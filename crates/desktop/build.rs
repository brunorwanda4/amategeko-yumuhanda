#[cfg(windows)]
fn main() {
    const ICON_PATH: &str = "../../assets/icon.ico";

    println!("cargo:rerun-if-changed={ICON_PATH}");

    let version = env!("CARGO_PKG_VERSION");
    let mut resource = winresource::WindowsResource::new();
    resource.set_icon(ICON_PATH);
    resource.set("ProductName", "Amategeko y’Umuhanda");
    resource.set("FileDescription", "Amategeko y’Umuhanda");
    resource.set("CompanyName", "Rwanda Bruno");
    resource.set("FileVersion", version);
    resource.set("ProductVersion", version);

    if let Err(error) = resource.compile() {
        panic!("failed to compile Windows resources: {error}");
    }
}

#[cfg(not(windows))]
fn main() {}
