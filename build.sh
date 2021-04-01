#!/bin/sh

# config ndk

# shellcheck disable=SC2039
source ./local/build_config.sh

cp ./local/cargo-config.toml ~/.cargo/config

# build
cargo build --target armv7-linux-androideabi --release

# optimize
"$NDK_PATH"/toolchains/arm-linux-androideabi-4.9/prebuilt/darwin-x86_64/bin/arm-linux-androideabi-strip -s target/armv7-linux-androideabi/release/$LIB_NAME

# move
SO_PATH="$PARALLEL_PATH"/src/main/jniLibs/armeabi-v7a
mkdir -vp "$SO_PATH"
cp target/armv7-linux-androideabi/release/"$LIB_NAME" "$SO_PATH"/"$LIB_NAME"