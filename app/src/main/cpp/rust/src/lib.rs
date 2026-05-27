use jni::errors::ThrowRuntimeExAndDefault;
use jni::objects::{Global, JClass, JObject, JValue};
use jni::strings::JNIString;
use jni::{EnvUnowned, JavaVM, jni_sig, jni_str};
use std::sync::{OnceLock};

static JVM: OnceLock<JavaVM> = OnceLock::new();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn JNI_OnLoad(
    vm: *mut jni::sys::JavaVM,
    _reserved: *mut std::ffi::c_void,
) -> i32 {
    let jvm = unsafe { JavaVM::from_raw(vm) };
    let _ = JVM.set(jvm);
    jni::sys::JNI_VERSION_1_6
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_com_example_rustroverdebugissue_MainActivity_stringFromJNI<'caller>(
    mut unowned_env: EnvUnowned<'caller>,
    _this: JObject<'caller>,
) -> JObject<'caller> {
    let outcome = unowned_env.with_env(|env: &mut jni::Env| {
        env.new_string("Hello from Rust").map(JObject::from)
    });

    outcome.resolve::<ThrowRuntimeExAndDefault>()
}
