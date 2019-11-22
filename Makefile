wasm_build:
    cd regen-client-wasm
    wasm-pack  build

build: wasm_build
    cargo build
