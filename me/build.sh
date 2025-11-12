#!/bin/bash

# Install Rust toolchain
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env

# Add wasm target
rustup target add wasm32-unknown-unknown

# Install Trunk and wasm-bindgen-cli
cargo install --locked trunk wasm-bindgen-cli

# Build your Yew app
trunk build --release
