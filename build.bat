del /S /Q dist && ^
cargo +nightly web deploy --target=wasm32-unknown-unknown --release && ^
wasm-gc target/deploy/rust.wasm -o opt/rust.gc.wasm && ^
wasm-opt -Os opt/rust.gc.wasm -o opt/rust.wasm && ^
npx parcel build target/deploy/index.html --public-url ./ && ^
copy opt\rust.wasm dist