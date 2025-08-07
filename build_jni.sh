#!/bin/bash
echo "Building JNI library..."

# Build for the current platform
cargo build --release --features jni

# Copy the library to Java project
case "$(uname -s)" in
    Darwin*)
        cp target/release/libcommonmeta.dylib java/src/main/resources/
        ;;
    Linux*)
        cp target/release/libcommonmeta.so java/src/main/resources/
        ;;
    MINGW*|CYGWIN*)
        cp target/release/commonmeta.dll java/src/main/resources/
        ;;
esac

echo "JNI build complete!"
