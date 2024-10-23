cargo lipo --package game_core --release
cbindgen --config game_core/cbindgen.toml --crate game_core --output game_core.h
rm -rf ios/Rust/*
cp target/aarch64-apple-ios/release/libgame_core.a ios/Rust
cp game_core.h ios/Rust