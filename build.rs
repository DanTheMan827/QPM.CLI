use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    // Generate the default 'cargo:' instruction output
    // Emit the instructions
    #[cfg(feature = "cli")]
    {
        use vergen::EmitBuilder;

        EmitBuilder::builder()
            .all_git()
            .emit()
            .expect("vergen failed");
    }

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        // Force rerun if Cargo.toml changes
        println!("cargo:rerun-if-changed=Cargo.toml");

        let cargo_version = env!("CARGO_PKG_VERSION"); // e.g. "1.2.3-alpha+abc123"

        // Strip build metadata and pre-release (take just 1.2.3)
        let base_version = cargo_version
            .split(|c| c == '-' || c == '+')
            .next()
            .unwrap(); // "1.2.3"

        let win_version = format!("{}.0", base_version); // "1.5.1.0"

        let mut res = winresource::WindowsResource::new();
        res.set_icon("icon.ico");
        res.set("CompanyName", "Quest Package Manager");
        res.set("FileDescription", "Quest Package Manager (QPM) is a command-line tool for managing and creating Quest mod projects. It functions as a package manager specifically designed for these projects, simplifying the process of handling mods and their dependencies.");
        res.set("FileVersion", &win_version);
        res.set("InternalName", "qpm");
        res.set("OriginalFilename", "qpm.exe");
        res.set("ProductName", "Quest Package Manager");
        res.set("ProductVersion", &win_version);

        res.compile().expect("Resource compilation failed");
    }

    Ok(())
}
