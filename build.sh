#!/usr/bin/env bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
ls -al target/wasm32-unknown-unknown/release/bil.wasm
# TODO: optimize / strip wasm
ls -al target/wasm32-unknown-unknown/release/bil.wasm
rm -rf build
mkdir build
cp target/wasm32-unknown-unknown/release/bil.wasm build/
cp -R assets build/
cp web/* build/
