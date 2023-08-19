use flapigen::{JavaConfig, LanguageConfig};
use rifgen::{Language, TypeCases};
use std::{env, path::Path};

fn main() {
    let java_lib_dir = Path::new("app")
        .join("src")
        .join("main")
        .join("java")
        .join("com")
        .join("oppzippy")
        .join("openscq30")
        .join("lib")
        .join("bindings");

    let rust_src_dir = Path::new("src");
    let glue_file = Path::new("glue.rs.in");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_file = Path::new(&out_dir).join("java_glue.rs");

    rifgen::Generator::new(TypeCases::CamelCase, Language::Java, vec![rust_src_dir])
        .generate_interface(glue_file);

    let swig_gen = flapigen::Generator::new(LanguageConfig::JavaConfig(
        JavaConfig::new(
            java_lib_dir,
            "com.oppzippy.openscq30.lib.bindings".to_string(),
        )
        .use_null_annotation_from_package("androidx.annotation".to_string()),
    ))
    .register_class_attribute_callback("PartialEq", |code, class_name| {
        let needle = format!("class {} {{", class_name);
        let class_pos = code
            .windows(needle.len())
            .position(|window| window == needle.as_bytes())
            .expect("Can not find begin of class");
        let insert_pos = class_pos + needle.len();
        code.splice(
            insert_pos..insert_pos,
            format!(
                r#"
    @Override
    public boolean equals(Object obj) {{
        if (obj instanceof {class})
            return (({class})obj).rustEq(this);
        return false;
    }}
"#,
                class = class_name
            )
            .as_bytes()
            .iter()
            .copied(),
        );
    })
    .rustfmt_bindings(true)
    .remove_not_generated_files_from_output_directory(true);
    swig_gen.expand("android bindings", glue_file, out_file);
    println!("cargo:rerun-if-changed=src");
}
