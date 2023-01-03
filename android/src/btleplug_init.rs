#[cfg(target_os = "android")]
use std::ffi::c_void;

// btleplug expects a JNIEnv from the "jni" library, but flapigen uses "jni-sys", so this needs to be completely
// separate from the flapigen code
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn Java_com_oppzippy_openscq30_BtleplugInitializer_init(
    env: jni::JNIEnv,
    _res: *const c_void,
) {
    btleplug::platform::init(&env).unwrap()
}
