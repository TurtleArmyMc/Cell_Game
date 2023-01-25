#!/bin/sh
cargo build --release -p game_wasm --target wasm32-unknown-unknown &&
wasm-bindgen target/wasm32-unknown-unknown/release/game_wasm.wasm --remove-name-section --remove-producers-section --no-typescript --target web --out-dir www/static/cell_game --out-name cell_game
