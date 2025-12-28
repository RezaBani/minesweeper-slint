use std::env;

use slint_build::CompileError;

fn main() -> Result<(), CompileError> {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }
    let input_files = ["./src/mine_sweeper_ui/main_window.slint"];

    for input_file in input_files {
        slint_build::compile(input_file)?;
        println!("cargo::rerun-if-changed={input_file}");
    }
    println!("cargo::rerun-if-changed=Cargo.toml");
    Ok(())
}
