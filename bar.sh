#!/usr/bin/env bash

error() {
    echo "$1"
    exit 1
}

cargo test --lib -- --nocapture || error "test failed"
wasm-pack test --chrome || "wasm-pack test"
wasm-pack build --target web || error "wasm-pack build"
# python -m http.server
