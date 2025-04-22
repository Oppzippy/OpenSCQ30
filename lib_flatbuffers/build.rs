use std::{io::Write, path::Path, process::Command};

use glob::glob;

fn main() {
    println!("cargo:rerun-if-changed=flatbuffers/**/*.fbs");
    let in_files = glob("./flatbuffers/*.fbs")
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let out_path = Path::new("src/generated");

    let flatc_output = Command::new("flatc")
        .arg("--rust")
        .args(["--filename-suffix", ""])
        .arg("-o")
        .arg(out_path)
        .args(&in_files)
        .output()
        .expect("failed to execute flatc");
    if let Err(err) = std::io::stdout().write(&flatc_output.stdout) {
        eprintln!("failed to forward stdout output from flatc: {err:?}");
    }
    if let Err(err) = std::io::stderr().write(&flatc_output.stderr) {
        eprintln!("failed to forward stderr output from flatc: {err:?}");
    }
    assert!(
        flatc_output.status.success(),
        "failed to generate rust flatbuffers code",
    );

    let mod_rs = in_files
        .iter()
        .map(|path| path.file_stem().unwrap().to_str().unwrap())
        .map(|name| format!("pub mod {name};\n"))
        .collect::<String>();
    std::fs::write(Path::new("src/generated/mod.rs"), mod_rs)
        .expect("failed to write generated mod.rs");
}
