rmdir /S /Q .\out
@REM cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "DepthLudum" ./target/wasm32-unknown-unknown/release/DepthLudum.wasm
xcopy /E /I assets\ .\out\assets\
copy .\index.html out\index.html