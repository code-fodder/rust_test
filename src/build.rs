fn main() {
    println!("cargo:rustc-link-lib=dylib=add_x64Linuxd");
    println!("cargo:rustc-link-search=native=/home/user/bbb/development/add_clib/lib");
}

