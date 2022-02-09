# 5D Brainfuck With Multiverse Time Travel

A visual interpreter for [5D Brainfuck With Multiverse Time Travel](https://esolangs.org/wiki/5D_Brainfuck_With_Multiverse_Time_Travel).

## Running the Project

```bash

# make sure you have the wasm32-unknown-unknown target installed
rustup target add wasm32-unknown-unknown 

# you'll also need te trunk build tool
cargo install trunk wasm-bindgen-cli

# project will be served at localhost
trunk serve
```

## TODOs

- [ ] project details
- [x] parser
- [x] interpreter
- [x] web design (simple)
- [x] site deployed on [netlify](https://bf5d.netlify.app/)
