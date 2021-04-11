#![cfg(target_os = "android")]
#![allow(non_snake_case)]

// pub unsafe extern fn Java_com_parallel_android_ParallelEngine_cancel(_env: JNIEnv, _: JObject) {}
//
// pub unsafe extern fn Java_com_parallel_android_ParallelEngine_isCanceled(_env: JNIEnv, _: JObject) -> jboolean {}
//
// pub unsafe extern fn Java_com_parallel_android_ParallelEngine_isExecuted(_env: JNIEnv, _: JObject) -> jboolean {}
//
// pub unsafe extern fn Java_com_parallel_android_ParallelEngine_nativeEnqueue(_env: JNIEnv, _: JObject) {}

use jni::JNIEnv;
use jni::objects::{JObject, JString, JValue};

use parallel_net::engine::{request};

#[no_mangle]
pub unsafe extern fn Java_com_parallelnet_android_DemoFragment_req(env: JNIEnv, jni_obj: JObject) {
    android_log::init("woshishui");
    request(String::from("123"));
}