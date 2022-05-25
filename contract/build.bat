@echo off

cargo build --workspace --target wasm32-unknown-unknown --release
:: /Y - forced overwrite
xcopy %CD%\target\wasm32-unknown-unknown\release\*.wasm %CD%\out\main.wasm /Y
near dev-deploy