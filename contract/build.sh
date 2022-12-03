cargo build --workspace --target wasm32-unknown-unknown --release
rm out/*.wasm
cp -f target/wasm32-unknown-unknown/release/*.wasm out/main.wasm