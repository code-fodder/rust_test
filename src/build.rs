fn main() {
    println!("cargo:rustc-link-lib=dylib=add_x64Linuxd");
    println!("cargo:rustc-link-search=native=cadd/lib");
}

