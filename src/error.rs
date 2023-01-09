#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[cfg(target_os = "android")]
    #[error(transparent)]
    JniError(#[from] jni::errors::Error),
    #[cfg(target_os = "windows")]
    #[error(transparent)]
    WindowsError(#[from] windows::core::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
