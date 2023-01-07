use crate::{app_paths::AppPaths, error::Result};

static DATA_DIR_NAME: &str = "data";

pub struct AppPathsImpl<'a> {
    context: jni::objects::JObject<'a>,
    vm: jni::JavaVM,
}

impl<'a> AppPathsImpl<'a> {
    pub fn create() -> Result<Self> {
        let ctx = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
        let context = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };
        Ok(Self { context, vm })
    }

    fn get_env(&self) -> Result<jni::AttachGuard> {
        Ok(self.vm.attach_current_thread()?)
    }
}

impl<'a> AppPaths for AppPathsImpl<'a> {
    fn get_data_dir(&self) -> Result<std::path::PathBuf> {
        let env = self.get_env()?;
        let ctx_class = env.find_class("android/content/Context")?;
        let private_mode = env.get_static_field(ctx_class, "MODE_PRIVATE", "I")?;
        let dir_file = env
            .call_method(
                self.context,
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
}
