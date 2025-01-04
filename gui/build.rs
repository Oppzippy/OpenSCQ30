fn main() {
    #[cfg(target_os = "windows")]
    windows();

    glib_build_tools::compile_resources(
        &["src/ui"],
        "src/ui/widgets.gresource.xml",
        "widgets.gresource",
    );
    println!("cargo:rerun-if-changed=src");
}

#[cfg(target_os = "windows")]
fn windows() {
    use std::fs;
    use std::io::Write;
    use std::path::Path;

    let version = env!("CARGO_PKG_VERSION");
    let version_parts = parse_version(version);

    // We need to get the version information into the resources.rc file, but modifying it directly isn't ideal
    let resource_dir = Path::new("resources");
    let resources = fs::read_to_string(resource_dir.join("resources.rc")).unwrap();
    let resources_with_version = resources
        .replace("${VERSION}", version)
        .replace("${VERSION_MAJOR}", version_parts.0)
        .replace("${VERSION_MINOR}", version_parts.1)
        .replace("${VERSION_PATCH}", version_parts.2);

    // The file with text replacements needs to be in the same directory so relative path references work
    // It should get deleted, but it's in .gitignore in case it isn't.
    let mut temp_resource_file = tempfile::Builder::new().tempfile_in(resource_dir).unwrap();
    write!(temp_resource_file, "{}", resources_with_version).unwrap();
    temp_resource_file.flush().unwrap();

    embed_resource::compile(temp_resource_file.path(), embed_resource::NONE)
        .manifest_required()
        .unwrap();
    println!("cargo:rerun-if-changed=resources");
}

#[cfg(target_os = "windows")]
fn parse_version(version: &str) -> (&str, &str, &str) {
    use regex::Regex;

    let version_parts = Regex::new("([0-9]+)\\.([0-9]+)\\.([0-9]+)")
        .unwrap()
        .captures(version)
        .unwrap();
    let version_major = version_parts.get(1).unwrap().as_str();
    let version_minor = version_parts.get(2).unwrap().as_str();
    let version_patch = version_parts.get(3).unwrap().as_str();

    (version_major, version_minor, version_patch)
}
