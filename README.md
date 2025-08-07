# commonmeta parser
for Kromer use internally but should work with anything else

### how to use:
use one of the bindings, or rust. no docs yet, read the src.
the rust src has tests, same with the typescript one. java is the exact same

It is published at https://npmjs.com/package/commonmeta and also at https://repo.sad.ovh. Maven instructions soonTM.

### how 2 build
#### build for rust
just include lib.rs dude

#### build for WASM (typescript)
install wasm-pack
`cargo install wasm-pack` or `npm install -g wasm-pack` (binaries)
run `./build_wasm.sh`, `npm publish` in `typescript/`

#### build for java
run `./build_jni.sh`, `./gradlew publish` in `java/` (publishes to repo.sad.ovh)
