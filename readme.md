# Rust Test Project

## 1. Tooling up and building

### Install Cargo Modules

- Recursive clean: `cargo install cargo-clean-recursive`
- Create rust FFI bindings to C libs: `cargo install bindgen`

### Rustup - Add targets

- show all targets: `rustup target list`
- add a target: `rustup target add <target-name>` (now do `cargo build --target <tab>` you can see your new target listed)

### Building

- local debug target: `cargo build`
- local release target: `cargo build --release`
- arm7 (bbb) debug target: `cargo build --target armv7-unknown-linux-gnueabihf`

### Clean

- clean all targets: `cargo clean`
- clean release only: `cargo clean --release`

### Run

- Run the compiled (local target): `cargo run`

### Test

- run all tests: `cargo test`
- run all tests with the word "main" in the function: `cargo test main`
- run test function main_t1 only: `cargo test main_t1 -- --exact`

## 2. Language features

### Templates

Template functions are much harder to write, there is no weak typing so fully generic code is not easy - but is somewhat possible.
See: [simple-rust-generic-template-add-function](https://stackoverflow.com/questions/63748118/simple-rust-generic-template-add-function)

## 3. Project Structuring

The project structure looks like this:

```shell
├── build.rs
├── cadd
│   ├── cutils.cpp
│   ├── cutils.h
│   ├── cutils.hpp
│   ├── makefile
├── Cargo.toml
├── readme.md
├── src
│   └── main.rs
└── utils
    ├── Cargo.toml
    └── src
        └── lib.rs
```

Where:

- `cadd` is a small pure c/c++ library (.so) -> `libadd_x64Linuxd.so*`
- `utils` is a small rust sub-package (library) -> `libutils.rlib`
- The top level is a rust executable, which imports utils and links to the c-lib libadd -> `rust_test`

### Dynamic linkage of the c-library

In this folder there is a sub-folder called "cadd" which is a c/c++ shared library project which compiles with `make`. I have linked this library into the project and tagged (with comments) each place I needed to edit in order to use it with `EXTERN_C`. Search the code and toml files for this tag.

For the moment you need to build the cadd lib first then run `cargo build`. But I hope to be able to add that step into the build script (`build.rs`).

When running the executable `rust_test` it needs to be able to find the c-library in the `cadd/lib/` folder relative to the top-level (or set the `LD_LIBRARY_PATH` accordingly).

### Dynamic/static linkage of the rust-library

I have not quite figured out how to build a hierarchical structure containing .rlib and executables that link dynamically. However this repo does have a sub-folder (could be a submodule) called utils that builds to a .rlib - but when built from the top level just gets linked statically into the top-level project.
