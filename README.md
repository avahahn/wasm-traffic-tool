# WASMAPP

Swiss army knife traffic debugging type thing.
Current proxy.wasm comes from wasmtime 18.
Comes with an NGINX Unit config.

```
$ cargo build --target wasm32-wasi
$ wasm-tools component new ./target/wasm32-wasi/debug/wasmapp.wasm \
    --adapt ./wasi_snapshot_preview1.proxy.wasm \
    -o wasmapp-proxy-component.wasm
```
