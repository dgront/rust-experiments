#cargo build --target wasm32-unknown-unknown --release

wasm-pack build --target web --no-typescript

wasm-gc pkg/create_svg_bg.wasm
