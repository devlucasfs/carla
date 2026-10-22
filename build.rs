use std::{env, path::PathBuf};

fn main() {
    let home = env::var("HOME").unwrap();
    let carla_bin = PathBuf::from(home).join(".carla/bin");

    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-search=native={}", carla_bin.display());

    println!("cargo:rustc-link-lib=dylib=morgana");
    println!("cargo:rustc-link-lib=dylib=runa");
    println!("cargo:rustc-link-lib=dylib=eva");

    cc::Build::new()
        .file("csrc/runa_manager.c")
        .include("csrc")
        .flag_if_supported("-Wall")
        .opt_level(2)
        .compile("runa_manager");

    println!("cargo:rerun-if-changed=csrc/");
}
