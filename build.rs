use std::{env, path::PathBuf};

use slint_build::CompileError;

fn main() -> Result<(), CompileError> {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let input_path = PathBuf::from("./src/ui");

    let input_files: Vec<_> = input_path
        .read_dir()
        .unwrap()
        .into_iter()
        .filter_map(|fi| {
            let path = fi.unwrap().path();
            if path.is_file() && path.to_string_lossy().ends_with(".slint") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    for input_file in input_files {
        slint_build::compile(&input_file)?;
        println!("cargo::rerun-if-changed={}", input_file.to_string_lossy());
    }
    println!("cargo::rerun-if-changed=Cargo.toml");
    Ok(())
}
