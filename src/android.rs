use crate::error::Result;

static DATA_DIR_NAME: &str = "data";

macro_rules! init_jni {
    ($env:ident, $context:ident) => {
        let ctx = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
        let $context = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };
        let $env = vm.attach_current_thread()?;
    };
}

pub fn get_data_dir() -> Result<std::path::PathBuf> {
    init_jni!(env, context);
    let ctx_class = env.find_class("android/content/Context")?;
    let private_mode = env.get_static_field(ctx_class, "MODE_PRIVATE", "I")?;
    let dir_file = env
        .call_method(
            context,
            "getDir",
            "(Ljava/lang/String;I)Ljava/io/File;",
            &[env.new_string(DATA_DIR_NAME)?.into(), private_mode],
        )?
        .l()?;
    let dir_jstr = env
        .call_method(dir_file, "getAbsolutePath", "()Ljava/lang/String;", &[])?
        .l()?;
    let dir_str: String = env.get_string(dir_jstr.into())?.into();
    Ok(dir_str.into())
}
