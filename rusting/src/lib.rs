extern crate jni;

use jni::JNIEnv;
use jni::objects::{JClass, JString, JObject};
use jni::sys::jstring;
use jni::sys::jint;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_github_marmalade_rusting_RustingScala_00024_fact(_env: JNIEnv, _class: JClass, n: jint) -> jint {
    (1..n+1).fold(1, |p, n| p*n)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_github_marmalade_rusting_RustingScala_00024_greetings(env: JNIEnv, _class: JObject, name: JString) -> jstring {
    let input: String = env.get_string(name).expect("Couldn't get java string!").into();
    let output = env.new_string(format!("Hello, {}!", input)).expect("Couldn't create java string!");
    output.into_inner()
}
