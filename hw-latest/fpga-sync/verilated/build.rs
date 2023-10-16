

use std::{
    ffi::OsStr,
    fs::File,
    io::{BufRead, BufReader},
    iter,
    path::{Path, PathBuf},
    process,
};

fn cmd_args(cmd: &mut process::Command) -> Vec<&OsStr> {
    iter::once(cmd.get_program())
        .chain(cmd.get_args())
        .collect()
}

fn run_command(cmd: &mut process::Command) {
    match cmd.status() {
        Err(err) => {
            eprintln!("Command {:?} failed: {}", cmd_args(cmd), err);
            std::process::exit(1);
        }
        Ok(status) => {
            if !status.success() {
                eprintln!("Command {:?} exit code {:?}", cmd_args(cmd), status.code());
                eprintln!("Please ensure that you have verilator 5.004 or later installed");
                std::process::exit(1);
            }
        }
    }
}

fn main() {
    if std::env::var_os("CARGO_FEATURE_VERILATOR").is_none() {
        return;
    }
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let mut make_cmd = process::Command::new("make");
    make_cmd.current_dir(&manifest_dir);
    if std::env::var_os("CARGO_FEATURE_ITRNG").is_some() {
        make_cmd.arg("EXTRA_VERILATOR_FLAGS=-DCALIPTRA_INTERNAL_TRNG");
    }

    run_command(&mut make_cmd);
    println!("cargo:rustc-link-search={}/out", manifest_dir.display());
    println!("cargo:rustc-link-lib=static=caliptra_fpga_sync_verilated");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}

