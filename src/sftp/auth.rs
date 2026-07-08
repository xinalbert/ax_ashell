use std::sync::Arc;

use anyhow::{Context, Result, anyhow};
use russh::{
    Disconnect,
    client::{self, Handler},
    keys::{HashAlg, PrivateKey, decode_secret_key, key::PrivateKeyWithHashAlg, load_secret_key},
};

use crate::session::config::{AuthMethod, Session};

use super::path::expand_key_path;

pub(super) async fn connect_and_authenticate(
    session: &Session,
) -> Result<Arc<russh::client::Handle<SftpClientHandler>>> {
    let config = Arc::new(client::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(600)),
        keepalive_interval: Some(std::time::Duration::from_secs(3)),
        keepalive_max: 2,
        ..Default::default()
    });
    let addr = format!("{}:{}", session.host, session.port);
    let stream = crate::session::config::connect_proxy(session).await?;
    let mut handle = client::connect_stream(config, stream, SftpClientHandler)
        .await
        .with_context(|| format!("connect {addr} failed"))?;

    let authed = match session.auth {
        AuthMethod::Password => handle
            .authenticate_password(&session.user, &session.password)
            .await
            .context("password authentication failed")?
            .success(),
        AuthMethod::Key => {
            let keypair = load_session_private_key(session)?;
            let keys = private_keys_with_algs(keypair).context("invalid private key")?;
            let mut success = false;
            for key in keys {
                match handle.authenticate_publickey(&session.user, key).await {
                    Ok(result) if result.success() => {
                        success = true;
                        break;
                    }
                    Ok(_) => {
                        tracing::debug!(
                            "[sftp] public key auth failed with algorithm, trying next"
                        );
                        continue;
                    }
                    Err(e) => {
                        tracing::debug!("[sftp] public key auth error: {:?}, trying next", e);
                        continue;
                    }
                }
            }
            if !success {
                return Err(anyhow!(
                    "public key authentication failed for {}@{}:{}",
                    session.user,
                    session.host,
                    session.port
                ));
            }
            success
        }
    };

    if !authed {
        let _ = handle
            .disconnect(Disconnect::ByApplication, "auth failed", "")
            .await;
        return Err(anyhow!(
            "authentication failed: server rejected {} authentication for {}@{}:{}",
            match session.auth {
                AuthMethod::Password => "password",
                AuthMethod::Key => "public key",
            },
            session.user,
            session.host,
            session.port
        ));
    }

    Ok(Arc::new(handle))
}

fn load_session_private_key(session: &Session) -> Result<PrivateKey> {
    let inline_key = normalize_inline_private_key(&session.private_key_inline);
    let key_path = expand_key_path(session.private_key_path.trim());
    let passphrase = session.passphrase.trim();
    let passphrase = (!passphrase.is_empty()).then_some(passphrase);
    let has_inline = !inline_key.is_empty();
    let has_path = key_path.is_some();

    if !has_inline && !has_path {
        return Err(anyhow!("private key content or path is required"));
    }

    let mut errors = Vec::new();

    if has_inline {
        match decode_secret_key(&inline_key, passphrase) {
            Ok(key) => return Ok(key),
            Err(err) => errors.push(format!("decode private key content: {err}")),
        }
    }

    if let Some(path) = key_path {
        match load_secret_key(path.as_path(), passphrase) {
            Ok(key) => return Ok(key),
            Err(err) => errors.push(format!("load key {}: {err}", path.display())),
        }
    }

    Err(anyhow!(errors.join("; ")))
}

fn private_keys_with_algs(keypair: PrivateKey) -> Result<Vec<PrivateKeyWithHashAlg>> {
    let mut algs = Vec::new();
    let key_arc = Arc::new(keypair);

    if key_arc.algorithm().is_rsa() {
        algs.push(PrivateKeyWithHashAlg::new(
            key_arc.clone(),
            Some(HashAlg::Sha512),
        ));
        algs.push(PrivateKeyWithHashAlg::new(
            key_arc.clone(),
            Some(HashAlg::Sha256),
        ));
        algs.push(PrivateKeyWithHashAlg::new(key_arc.clone(), None));
    } else {
        algs.push(PrivateKeyWithHashAlg::new(key_arc.clone(), None));
    }

    if algs.is_empty() {
        return Err(anyhow!(
            "Failed to construct PrivateKeyWithHashAlg for any supported hash algorithm"
        ));
    }

    Ok(algs)
}

fn normalize_inline_private_key(value: &str) -> String {
    let mut normalized = value
        .trim()
        .replace("\\r\\n", "\n")
        .replace("\\n", "\n")
        .replace("\r\n", "\n");
    if !normalized.ends_with('\n') {
        normalized.push('\n');
    }
    normalized
}

#[derive(Clone)]
pub(super) struct SftpClientHandler;

impl Handler for SftpClientHandler {
    type Error = anyhow::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &russh::keys::ssh_key::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
}
