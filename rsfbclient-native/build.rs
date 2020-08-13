fn main() {
    #[cfg(feature = "linking")]
    // println!("cargo:rustc-link-lib=dylib=fbclient");
    println!("cargo:rustc-link-lib=fbclient");
}
