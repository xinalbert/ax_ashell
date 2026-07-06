fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icons/terminal_icon_all_formats/terminal_icon.ico");
        res.compile().unwrap();
    }
}
