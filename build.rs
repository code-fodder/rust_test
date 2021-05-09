fn main() {
    // The name of the library to link to, i.e. like: -l<lib>
    println!("cargo:rustc-link-lib=dylib=add_x64Linuxd");
    // The library search path for linking, i.e. like -L<path>
    println!("cargo:rustc-link-search=native=cadd/lib");
    // The run-time library search path (LD_LIBRARY_PATH)
    println!("cargo:rustc-env=LD_LIBRARY_PATH=cadd/lib");
}

