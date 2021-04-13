#!/bin/sh

# config ndk
SO_PATH=../jniLibs/armeabi-v7a
LIB_NAME="libparallel_net_android.so"
NDK_PATH=/Users/wangyucheng/Library/Android/sdk/ndk/22.1.7171670

# shellcheck disable=SC2039
cp ../../../../../../../local/cargo-config.toml ~/.cargo/config

# build
cargo build --target armv7-linux-androideabi --release

# optimize
"$NDK_PATH"/toolchains/arm-linux-androideabi-4.9/prebuilt/darwin-x86_64/bin/arm-linux-androideabi-strip -s ./target/armv7-linux-androideabi/release/"$LIB_NAME"

# move
cp ./target/armv7-linux-androideabi/release/"$LIB_NAME" "$SO_PATH"/"$LIB_NAME"