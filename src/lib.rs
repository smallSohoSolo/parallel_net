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
use jni::objects::{JObject, JValue};

#[no_mangle]
pub unsafe extern fn Java_com_parallelnet_android_MainActivity_test(env: JNIEnv, jobj: JObject) {
    env.call_method(jobj, "ttt", "(I)V", &[JValue::Object(env.new_string("ttt"))]);
}