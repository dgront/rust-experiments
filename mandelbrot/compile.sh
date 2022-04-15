#cargo build --target wasm32-unknown-unknown --release

wasm-pack build --target web --no-typescript

wasm-gc pkg/mandelbrot_bg.wasm
