# Pong

A toy implementation of Pong, written in Rust.

Demo: https://sberkun.github.io/pong

## Building

Prerequisites: [Rust](https://rustup.rs/) and [wasm-pack](https://rustwasm.github.io/wasm-pack/)

Run `wasm-pack build --target web`. This should build the `pkg` files.

You can then serve the project with any http server, such as `python3 -m http.server`.