use std::{env, path::PathBuf};

fn main() {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }
    let input_path = PathBuf::from("./src/mine_sweeper_ui/main_window.slint");
    let input_path = match input_path.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            panic!("{e}");
        }
    };
    let output_path = PathBuf::from("./src/mine_sweeper_ui/main_window.rs");
    let output_path = match output_path.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            panic!("{e}");
        }
    };
    let input_files = match slint_build::compile_with_output_path(
        input_path,
        output_path,
        slint_build::CompilerConfiguration::new(),
    ) {
        Ok(input_path) => input_path,
        Err(e) => {
            panic!("{e}");
        }
    };
    for file in input_files {
        println!("cargo::rerun-if-changed={}", file.to_string_lossy());
    }
    println!("cargo::rerun-if-changed=Cargo.toml");
}
