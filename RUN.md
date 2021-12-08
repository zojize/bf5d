# BF5D

## Running the Project

```bash

# make sure you have the wasm32-unknown-unknown target installed
rustup target add wasm32-unknown-unknown 

# you'll also need te trunk build tool
cargo install trunk wasm-bindgen-cli

# project will be served at localhost
trunk serve
```
