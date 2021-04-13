#![cfg(target_os = "android")]
#![allow(non_snake_case)]

// pub unsafe extern fn Java_com_parallel_android_ParallelEngine_cancel(_env: JNIEnv, _: JObject) {}
//
// pub unsafe extern fn Java_com_parallel_android_ParallelEngine_isCanceled(_env: JNIEnv, _: JObject) -> jboolean {}
//
// pub unsafe extern fn Java_com_parallel_android_ParallelEngine_isExecuted(_env: JNIEnv, _: JObject) -> jboolean {}
//
// pub unsafe extern fn Java_com_parallel_android_ParallelEngine_nativeEnqueue(_env: JNIEnv, _: JObject) {}

use std::sync::Mutex;

use jni::JNIEnv;
use jni::objects::{JObject, JString, JValue};
use jni::sys::jlong;
use parallel_net::engine::{Engine, new_engine};
use hyper::body::Body;
use hyper::Error;
use hyper::http::response::Parts;

#[no_mangle]
pub unsafe extern "C" fn Java_com_parallelnet_android_DemoFragment_initParallel(env: JNIEnv, jni_obj: JObject) -> jlong {
    android_log::init("parallel_net").unwrap();
    let engine = new_engine();
    let container = Box::into_raw(Box::new(Mutex::new(engine))) as jlong;
    container
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_parallelnet_android_DemoFragment_drop(env: JNIEnv, jni_obj: JObject, engine: jlong) {
    let _drop = Box::from_raw(engine as *mut Mutex<Engine>);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_parallelnet_android_DemoFragment_req(env: JNIEnv, jni_obj: JObject, engine: jlong) {
    let engine = engine as *const Mutex<Engine>;
    let lock = (*engine).lock().unwrap();
    lock.request(|parts: Parts, body_string: String| {
        log::debug!("{}", body_string);
    }, |err: Error| {});
}