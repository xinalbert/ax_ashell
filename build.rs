fn main() {
    emit_public_version();

    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icons/terminal_icon_all_formats/terminal_icon.ico");
        res.compile().unwrap();
    }
}

fn emit_public_version() {
    println!("cargo:rerun-if-env-changed=RELEASE_PUBLIC_VERSION");

    let Some(version) = std::env::var("RELEASE_PUBLIC_VERSION")
        .ok()
        .map(|version| version.trim().to_string())
        .filter(|version| !version.is_empty())
    else {
        return;
    };

    println!("cargo:rustc-env=AXSHELL_PUBLIC_VERSION={version}");
}
