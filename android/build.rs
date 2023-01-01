use flapigen::{JavaConfig, LanguageConfig};
use rifgen::{Language, TypeCases};
use std::{env, path::Path};

fn main() {
    let java_source_dir = Path::new("app")
        .join("src")
        .join("main")
        .join("java")
        .join("com")
        .join("oppzippy")
        .join("openscq30");

    let java_lib_dir = java_source_dir.join("lib");

    let rust_src_dir = Path::new("src");
    let glue_file = rust_src_dir.join("glue.rs.in");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    rifgen::Generator::new(TypeCases::CamelCase, Language::Java, &rust_src_dir)
        .generate_interface(&glue_file);

    let swig_gen = flapigen::Generator::new(LanguageConfig::JavaConfig(
        JavaConfig::new(
            java_lib_dir.to_owned(),
            "com.oppzippy.openscq30.lib".to_string(),
        )
        .use_null_annotation_from_package("androidx.annotation".to_string()),
    ))
    .rustfmt_bindings(true)
    .remove_not_generated_files_from_output_directory(true);
    swig_gen.expand("android bindings", &glue_file, out_dir.join("java_glue.rs"));
    println!("cargo:rerun-if-changed=src");
}
