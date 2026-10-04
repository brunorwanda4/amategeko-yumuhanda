//! Embeds the Windows app icon into the executable.

fn main() {
    println!("cargo:rerun-if-changed=resources/app.rc");
    println!("cargo:rerun-if-changed=resources/app.ico");

    // Only Windows targets have exe resources; other targets need nothing.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    // A missing resource compiler only loses the icon, so warn instead of failing.
    if let Err(err) =
        embed_resource::compile("resources/app.rc", embed_resource::NONE).manifest_optional()
    {
        println!("cargo:warning=app icon not embedded: {err}");
    }
}
