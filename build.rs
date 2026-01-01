use std::{env, path::PathBuf};

use slint_build::CompileError;

fn main() -> Result<(), CompileError> {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
        env::set_var("SLINT_BACKEND", "winit-software");
    }

    // slint can only compile one file
    let main_slint = "./src/ui/main.slint";
    slint_build::compile(main_slint)?;

    // Checking if any slint file has changed then invoke build process
    let ui_dir_path = PathBuf::from("./src/ui");
    let slint_files: Vec<_> = ui_dir_path
        .read_dir()
        .unwrap()
        .into_iter()
        .filter_map(|path_info| {
            let path = path_info.unwrap().path();
            if path.is_file() && path.to_string_lossy().ends_with(".slint") {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    for slint_file in slint_files {
        println!("cargo::rerun-if-changed={}", slint_file.to_string_lossy());
    }

    Ok(())
}
