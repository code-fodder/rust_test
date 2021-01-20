# Building
 - local debug target: `cargo build`
 - local release target: `cargo build --release`
 - arm7 (bbb) debug target: `cargo build --target armv7-unknown-linux-gnueabihf`

# Clean
 - clean all targets: `cargo clean`
 - clean release only: `cargo clean --release`

# Install Cargo Modules
 - Recursive clean: `cargo install cargo-clean-recursive`

# Rustup - Add targets
 - show all targets: `rustup target list`
 - add a target: `rustup target add <target-name>` (now do `cargo build --target <tab>` you can see your new target listed)
