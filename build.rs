fn main() {
    #[cfg(windows)] {
        use chrono::Datelike;

        let year = chrono::Utc::now().year();
        let name = "ChronoClean";
        let version = env!("CARGO_PKG_VERSION");

        let mut res = winresource::WindowsResource::new();
        res.set("FileVersion", &format!("{version}.0"));  // 0.1.0.0
        res.set("ProductVersion", version);                     // 0.1.0
        res.set("ProductName", name);
        res.set("FileDescription", name);
        res.set("OriginalFilename", &format!("{}.exe", env!("CARGO_PKG_NAME")));
        res.set("CompanyName", "SecretX33");
        res.set("LegalCopyright", &format!("Copyright © {year} SecretX33"));
        res.set_icon("icons/icon.ico");
        res.compile().unwrap();
    }
}