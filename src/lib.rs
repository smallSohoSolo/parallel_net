#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use jni::JNIEnv;
use jni::objects::JObject;
use jni::sys::jstring;

#[no_mangle]
pub extern fn Java_com_parallel_android_ParallelEngine_cancel(_env: JNIEnv, _: JObject) {}

#[no_mangle]
pub extern fn Java_com_parallelnet_android_MainActivity_hello(env: JNIEnv, _: JObject) -> jstring {
    env.new_string("66666").unwrap().into_inner()
}
