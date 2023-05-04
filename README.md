# rustman

Daniel Bauer (bauerda@pm.me)

Pac-Man clone written in Rust.

## Prerequisites

- [Git](https://git-scm.com "Git")
- [Rust](https://www.rust-lang.org/tools/install "Rust") (2021 edition)

### Additional Dependencies

#### Ubuntu 23.04

```
$ sudo apt install librust-alsa-sys-dev libudev-dev
```

## Build & Run

1. Clone project repository  
`git clone https://github.com/bauerdaniel/rustman.git`
2. Change to the source directory  
`cd rustman/rustman`
3. Compile the game using Cargo  
`cargo run --release`  
Note: Compilation will take some time since all optimizations are enabled.

## WASM

Alternatively, the game can also be run in the web browser using WebAssembly.

```
$ rustup target install wasm32-unknown-unknown
$ cargo install wasm-bindgen-cli
$ cargo build --release --target wasm32-unknown-unknown
$ wasm-bindgen --out-dir ./target/wasm/ --target web target/wasm32-unknown-unknown/release/rustman.wasm
$ cp -R assets target/wasm/assets && cp -R wasm target
$ python3 -m http.server --directory target/wasm
```