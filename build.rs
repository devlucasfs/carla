fn main() {
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-lib=dylib=morgana");
    println!("cargo:rustc-link-lib=dylib=eva");
}
