wasm-pack build --target web --no-pack --features wasm
rm -rf typescript/pkg
mv pkg typescript
