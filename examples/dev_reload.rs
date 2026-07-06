use std::{
    env,
    ffi::OsString,
    fs::{self, File},
    io::{self, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::{Child, Command, ExitStatus, Stdio},
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver},
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, bail};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};

const DEFAULT_WATCH_PATHS: &[&str] = &[
    "src",
    "assets",
    "locales",
    "Cargo.toml",
    "Cargo.lock",
    "build.rs",
    ".cargo",
];

fn main() {
    if let Err(err) = run() {
        eprintln!("[dev-reload] {err:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let config = Config::parse(env::args().skip(1).collect())?;
    if config.show_help {
        print!("{}", Config::help());
        return Ok(());
    }

    let root = env::current_dir().context("resolve current directory")?;
    let mut runner = DevReload::new(root, config);
    runner.run()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Config {
    release: bool,
    debounce_ms: u64,
    watch_paths: Vec<PathBuf>,
    app_args: Vec<OsString>,
    show_help: bool,
}

impl Config {
    fn parse(args: Vec<String>) -> Result<Self> {
        let mut release = false;
        let mut debounce_ms = 400_u64;
        let mut watch_paths = Vec::new();
        let mut app_args = Vec::new();
        let mut show_help = false;

        let mut i = 0;
        let mut passthrough = false;
        while i < args.len() {
            let arg = &args[i];
            if passthrough {
                app_args.push(OsString::from(arg));
                i += 1;
                continue;
            }

            match arg.as_str() {
                "--" => {
                    passthrough = true;
                    i += 1;
                }
                "--release" => {
                    release = true;
                    i += 1;
                }
                "--debounce-ms" => {
                    let value = args.get(i + 1).context("missing value for --debounce-ms")?;
                    debounce_ms = value
                        .parse::<u64>()
                        .with_context(|| format!("invalid --debounce-ms value: {value}"))?;
                    i += 2;
                }
                "--watch" => {
                    let value = args.get(i + 1).context("missing value for --watch")?;
                    watch_paths.push(PathBuf::from(value));
                    i += 2;
                }
                "--help" | "-h" => {
                    show_help = true;
                    i += 1;
                }
                other => {
                    bail!("unknown argument: {other}\n\n{}", Self::help());
                }
            }
        }

        if watch_paths.is_empty() {
            watch_paths = DEFAULT_WATCH_PATHS.iter().map(PathBuf::from).collect();
        }

        Ok(Self {
            release,
            debounce_ms,
            watch_paths,
            app_args,
            show_help,
        })
    }

    fn help() -> &'static str {
        "\
Usage:
  cargo run --example dev_reload -- [options] [-- <ax_ashell-args>]
  cargo dev-reload [options] [-- <ax_ashell-args>]

Options:
  --release             Build and run target/release/ax_ashell
  --debounce-ms <ms>    Debounce file events before rebuild (default: 400)
  --watch <path>        Additional or replacement watch path; may be repeated
  -h, --help            Show this help

Notes:
  - This is restart-based development reload, not state-preserving hot reload.
  - On file change, the running app is stopped first, then rebuilt and restarted.
  - Default watch set: src, assets, locales, Cargo.toml, Cargo.lock, build.rs, .cargo
"
    }
}

struct DevReload {
    root: PathBuf,
    config: Config,
    child: Option<Child>,
    logs: Option<DebugLogs>,
}

#[derive(Clone)]
struct SharedLogFile {
    inner: Arc<Mutex<File>>,
}

impl SharedLogFile {
    fn create(path: PathBuf) -> Result<Self> {
        let file =
            File::create(&path).with_context(|| format!("create log file {}", path.display()))?;
        Ok(Self {
            inner: Arc::new(Mutex::new(file)),
        })
    }

    fn write_line(&self, line: impl AsRef<str>) -> Result<()> {
        let mut file = self
            .inner
            .lock()
            .map_err(|_| anyhow::anyhow!("log file lock poisoned"))?;
        writeln!(file, "{}", line.as_ref()).context("write log line")?;
        file.flush().context("flush log line")
    }

    fn write_all(&self, bytes: &[u8]) -> Result<()> {
        let mut file = self
            .inner
            .lock()
            .map_err(|_| anyhow::anyhow!("log file lock poisoned"))?;
        file.write_all(bytes).context("write log bytes")?;
        file.flush().context("flush log bytes")
    }
}

struct DebugLogs {
    dir: PathBuf,
    runner: SharedLogFile,
    build_stdout: SharedLogFile,
    build_stderr: SharedLogFile,
    app_stdout: SharedLogFile,
    app_stderr: SharedLogFile,
}

impl DebugLogs {
    fn create(root: &Path, target_dir: &Path) -> Result<Self> {
        let dir = target_dir.join("debug").join("dev-reload-logs");
        fs::create_dir_all(&dir)
            .with_context(|| format!("create debug log dir {}", dir.display()))?;

        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let session_dir = dir.join(format!("session-{stamp}"));
        fs::create_dir_all(&session_dir)
            .with_context(|| format!("create debug session dir {}", session_dir.display()))?;

        let logs = Self {
            dir: session_dir.clone(),
            runner: SharedLogFile::create(session_dir.join("dev-reload.log"))?,
            build_stdout: SharedLogFile::create(session_dir.join("cargo-build.stdout.log"))?,
            build_stderr: SharedLogFile::create(session_dir.join("cargo-build.stderr.log"))?,
            app_stdout: SharedLogFile::create(session_dir.join("ax_ashell.stdout.log"))?,
            app_stderr: SharedLogFile::create(session_dir.join("ax_ashell.stderr.log"))?,
        };
        logs.runner.write_line(format!(
            "[dev-reload] debug logs enabled under {} (root: {})",
            logs.dir.display(),
            root.display()
        ))?;
        Ok(logs)
    }
}

impl DevReload {
    fn new(root: PathBuf, config: Config) -> Self {
        let logs = if config.release {
            None
        } else {
            let target_dir = resolve_target_dir(&root);
            match DebugLogs::create(&root, &target_dir) {
                Ok(logs) => {
                    eprintln!("[dev-reload] debug logs: {}", logs.dir.display());
                    Some(logs)
                }
                Err(err) => {
                    eprintln!("[dev-reload] failed to initialize debug logs: {err:#}");
                    None
                }
            }
        };
        Self {
            root,
            config,
            child: None,
            logs,
        }
    }

    fn run(&mut self) -> Result<()> {
        let (tx, rx) = mpsc::channel();
        let mut watcher = self.build_watcher(tx)?;

        for watch_path in self.resolved_watch_paths() {
            let mode = if watch_path.is_dir() {
                RecursiveMode::Recursive
            } else {
                RecursiveMode::NonRecursive
            };
            watcher
                .watch(&watch_path, mode)
                .with_context(|| format!("watch {}", watch_path.display()))?;
            self.log_runner(format!("[dev-reload] watching {}", watch_path.display()));
        }

        self.rebuild_and_restart("initial start", true)?;

        loop {
            let events =
                self.collect_change_batch(&rx, Duration::from_millis(self.config.debounce_ms))?;
            let summary = summarize_events(&events);
            self.rebuild_and_restart(&summary, false)?;
        }
    }

    fn build_watcher(
        &self,
        tx: mpsc::Sender<notify::Result<Event>>,
    ) -> notify::Result<RecommendedWatcher> {
        notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        })
    }

    fn resolved_watch_paths(&self) -> Vec<PathBuf> {
        self.config
            .watch_paths
            .iter()
            .map(|path| self.root.join(path))
            .filter(|path| path.exists())
            .collect()
    }

    fn rebuild_and_restart(&mut self, reason: &str, fail_fast: bool) -> Result<()> {
        self.log_runner(format!("[dev-reload] trigger: {reason}"));
        if let Err(err) = self.build_app() {
            self.log_runner(format!("[dev-reload] build failed: {err:#}"));
            if fail_fast {
                return Err(err);
            }
            return Ok(());
        }

        self.stop_child()?;
        self.start_app()?;
        Ok(())
    }

    fn collect_change_batch(
        &self,
        rx: &Receiver<notify::Result<Event>>,
        debounce: Duration,
    ) -> Result<Vec<Event>> {
        let first = rx.recv().context("watch channel closed")??;
        let mut events = vec![first];
        loop {
            match rx.recv_timeout(debounce) {
                Ok(Ok(event)) => events.push(event),
                Ok(Err(err)) => self.log_runner(format!("[dev-reload] watcher error: {err}")),
                Err(mpsc::RecvTimeoutError::Timeout) => return Ok(events),
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    return Err(
                        io::Error::new(io::ErrorKind::BrokenPipe, "watch channel closed").into(),
                    );
                }
            }
        }
    }

    fn build_app(&self) -> Result<()> {
        let mut command = Command::new("cargo");
        command
            .current_dir(&self.root)
            .arg("build")
            .arg("--bin")
            .arg("ax_ashell");
        if self.config.release {
            command.arg("--release");
        }
        command.stdin(Stdio::null());
        if self.logs.is_some() {
            command.stdout(Stdio::piped());
            command.stderr(Stdio::piped());
        } else {
            command.stdout(Stdio::inherit());
            command.stderr(Stdio::inherit());
        }

        let mut child = command.spawn().context("spawn cargo build")?;
        if let Some(logs) = &self.logs {
            let stdout = child.stdout.take();
            let stderr = child.stderr.take();
            spawn_stream_tee(
                stdout,
                io::stdout(),
                logs.build_stdout.clone(),
                "cargo-build:stdout",
            );
            spawn_stream_tee(
                stderr,
                io::stderr(),
                logs.build_stderr.clone(),
                "cargo-build:stderr",
            );
        }
        let status = child.wait().context("wait cargo build")?;
        ensure_success(status, "cargo build")
    }

    fn start_app(&mut self) -> Result<()> {
        let executable = self.binary_path();
        let mut command = Command::new(&executable);
        command.current_dir(&self.root);
        command.args(&self.config.app_args);
        command.stdin(Stdio::inherit());
        if self.logs.is_some() {
            command.stdout(Stdio::piped());
            command.stderr(Stdio::piped());
        } else {
            command.stdout(Stdio::inherit());
            command.stderr(Stdio::inherit());
        }

        let mut child = command
            .spawn()
            .with_context(|| format!("start {}", executable.display()))?;
        if let Some(logs) = &self.logs {
            let stdout = child.stdout.take();
            let stderr = child.stderr.take();
            spawn_stream_tee(
                stdout,
                io::stdout(),
                logs.app_stdout.clone(),
                "ax_ashell:stdout",
            );
            spawn_stream_tee(
                stderr,
                io::stderr(),
                logs.app_stderr.clone(),
                "ax_ashell:stderr",
            );
        }
        self.log_runner(format!("[dev-reload] started {}", executable.display()));
        self.child = Some(child);
        Ok(())
    }

    fn stop_child(&mut self) -> Result<()> {
        let Some(mut child) = self.child.take() else {
            return Ok(());
        };

        if child.try_wait().context("poll child process")?.is_some() {
            return Ok(());
        }

        // Stop first, then rebuild, to avoid executable replacement issues on Windows.
        child.kill().context("stop running app")?;
        let _ = child.wait();
        self.log_runner("[dev-reload] stopped running app");
        Ok(())
    }

    fn binary_path(&self) -> PathBuf {
        let mut base = match env::var_os("CARGO_TARGET_DIR") {
            Some(path) => PathBuf::from(path),
            None => self.root.join("target"),
        };
        if base.is_relative() {
            base = self.root.join(base);
        }
        let profile = if self.config.release {
            "release"
        } else {
            "debug"
        };
        base.join(profile)
            .join(format!("ax_ashell{}", env::consts::EXE_SUFFIX))
    }

    fn log_runner(&self, message: impl AsRef<str>) {
        let message = message.as_ref();
        eprintln!("{message}");
        if let Some(logs) = &self.logs {
            let _ = logs.runner.write_line(message);
        }
    }
}

fn summarize_events(events: &[Event]) -> String {
    let mut labels = Vec::new();
    for path in events
        .iter()
        .flat_map(|event| event.paths.iter())
        .filter_map(|path| relative_label(path))
    {
        if !labels.contains(&path) {
            labels.push(path);
        }
        if labels.len() == 3 {
            break;
        }
    }

    if labels.is_empty() {
        format!("{} filesystem events", events.len())
    } else {
        format!("{} filesystem events ({})", events.len(), labels.join(", "))
    }
}

fn relative_label(path: &Path) -> Option<String> {
    let cwd = env::current_dir().ok()?;
    let relative = path.strip_prefix(cwd).unwrap_or(path);
    Some(relative.display().to_string())
}

fn resolve_target_dir(root: &Path) -> PathBuf {
    let mut base = match env::var_os("CARGO_TARGET_DIR") {
        Some(path) => PathBuf::from(path),
        None => root.join("target"),
    };
    if base.is_relative() {
        base = root.join(base);
    }
    base
}

fn spawn_stream_tee<W>(
    stream: Option<impl Read + Send + 'static>,
    mut sink: W,
    log_file: SharedLogFile,
    label: &'static str,
) where
    W: Write + Send + 'static,
{
    let Some(stream) = stream else {
        return;
    };

    thread::spawn(move || {
        let mut reader = BufReader::new(stream);
        let mut buf = [0_u8; 8192];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    let chunk = &buf[..n];
                    let _ = sink.write_all(chunk);
                    let _ = sink.flush();
                    let _ = log_file.write_all(chunk);
                }
                Err(err) => {
                    let _ = log_file.write_line(format!("[dev-reload] {label} read error: {err}"));
                    break;
                }
            }
        }
    });
}

fn ensure_success(status: ExitStatus, action: &str) -> Result<()> {
    if status.success() {
        Ok(())
    } else {
        bail!("{action} failed with status {status}");
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, DEFAULT_WATCH_PATHS};
    use std::path::PathBuf;

    #[test]
    fn parses_defaults() {
        let config = Config::parse(vec![]).expect("parse defaults");
        assert!(!config.release);
        assert_eq!(config.debounce_ms, 400);
        assert_eq!(
            config.watch_paths,
            DEFAULT_WATCH_PATHS
                .iter()
                .map(PathBuf::from)
                .collect::<Vec<_>>()
        );
        assert!(config.app_args.is_empty());
        assert!(!config.show_help);
    }

    #[test]
    fn parses_custom_options_and_passthrough_args() {
        let config = Config::parse(vec![
            "--release".into(),
            "--debounce-ms".into(),
            "900".into(),
            "--watch".into(),
            "src".into(),
            "--watch".into(),
            "README.md".into(),
            "--".into(),
            "--foo".into(),
            "bar".into(),
        ])
        .expect("parse custom args");

        assert!(config.release);
        assert_eq!(config.debounce_ms, 900);
        assert_eq!(
            config.watch_paths,
            vec![PathBuf::from("src"), PathBuf::from("README.md")]
        );
        assert_eq!(config.app_args, vec!["--foo", "bar"]);
    }

    #[test]
    fn rejects_unknown_args() {
        let err = Config::parse(vec!["--wat".into()]).expect_err("unknown arg should fail");
        let message = format!("{err:#}");
        assert!(message.contains("unknown argument"));
    }
}
