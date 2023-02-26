fn main() {
    #[cfg(target_os = "windows")]
    windows();

    glib_build_tools::compile_resources(
        &["src/widgets"],
        "src/widgets/widgets.gresource.xml",
        "widgets.gresource",
    );
    println!("cargo:rerun-if-changed=src");
}

#[cfg(target_os = "windows")]
fn windows() {
    use std::path::Path;

    let resource_dir = Path::new("resources");
    embed_resource::compile(resource_dir.join("resources.rc"));
    println!("cargo:rerun-if-changed=resources");
}
