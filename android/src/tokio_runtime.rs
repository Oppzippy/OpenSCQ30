use jni::{objects::GlobalRef, JNIEnv, JavaVM};
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use tokio::runtime::{Handle, Runtime};
use tracing::info;

lazy_static! {
    pub static ref RUNTIME: OnceCell<Runtime> = OnceCell::new();
    static ref JAVAVM: OnceCell<JavaVM> = OnceCell::new();
    static ref CLASS_LOADER: OnceCell<GlobalRef> = OnceCell::new();
}

pub fn get_handle() -> &'static Handle {
    RUNTIME.get().unwrap().handle()
}

pub fn create_runtime(env: &JNIEnv) {
    let new_runtime = {
        let class = env
            .find_class("com/nonpolynomial/btleplug/android/impl/Peripheral")
            .unwrap();
        JAVAVM.set(env.get_java_vm().unwrap());
        let thread = env
            .call_static_method(
                "java/lang/Thread",
                "currentThread",
                "()Ljava/lang/Thread;",
                &[],
            )
            .unwrap()
            .l()
            .unwrap();
        let class_loader = env
            .call_method(
                thread,
                "getContextClassLoader",
                "()Ljava/lang/ClassLoader;",
                &[],
            )
            .unwrap()
            .l()
            .unwrap();

        CLASS_LOADER.set(env.new_global_ref(class_loader).unwrap());
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .on_thread_start(|| {
                info!("WRAPPING NEW THREAD IN VM");
                let vm = JAVAVM.get().unwrap();

                // We now need to call the following code block via JNI calls. God help us.
                //
                //  java.lang.Thread.currentThread().setContextClassLoader(
                //    java.lang.ClassLoader.getSystemClassLoader()
                //  );
                info!("Adding classloader to thread");

                let new_env = vm.attach_current_thread_permanently().unwrap();

                let thread = new_env
                    .call_static_method(
                        "java/lang/Thread",
                        "currentThread",
                        "()Ljava/lang/Thread;",
                        &[],
                    )
                    .unwrap()
                    .l()
                    .unwrap();
                new_env
                    .call_method(
                        thread,
                        "setContextClassLoader",
                        "(Ljava/lang/ClassLoader;)V",
                        &[CLASS_LOADER.get().unwrap().as_obj().into()],
                    )
                    .unwrap();
                info!("Classloader added to thread");
            })
            .build()
            .unwrap()
    };
    RUNTIME.set(new_runtime);
}
