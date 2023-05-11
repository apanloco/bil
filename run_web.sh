#!/usr/bin/env bash
rustup target add wasm32-unknown-unknown
cargo install basic-http-server
cargo build --target wasm32-unknown-unknown --release
ls -al target/wasm32-unknown-unknown/release/bil.wasm
wasm-strip target/wasm32-unknown-unknown/release/bil.wasm
ls -al target/wasm32-unknown-unknown/release/bil.wasm
ln -sf target/wasm32-unknown-unknown/release/bil.wasm
basic-http-server .
