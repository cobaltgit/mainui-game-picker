deps:
    rustup target add aarch64-unknown-linux-gnu
    rustup target add armv7-unknown-linux-gnueabihf

release: deps
    cargo zigbuild --target armv7-unknown-linux-gnueabihf.2.17 --release
    cargo zigbuild --target aarch64-unknown-linux-gnu.2.17 --release
