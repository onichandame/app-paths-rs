use std::path::PathBuf;

pub use error::Error;
use error::Result;

#[cfg(target_os = "android")]
mod android;
mod error;
mod fallback;
#[cfg(target_os = "windows")]
mod windows;

/// Get the path of the directory where all the app data should reside.
///
/// This path generally lives as long as the current app installation, meaning
/// that this path may be removed if the app is un-installed
pub fn get_data_dir() -> Result<PathBuf> {
    let dir = {
        #[cfg(target_os = "android")]
        {
            android::get_data_dir()?
        }
        #[cfg(target_os = "windows")]
        {
            crate::windows::get_data_dir()?
        }
        #[cfg(not(any(target_os = "android", target_os = "windows")))]
        {
            fallback::get_data_dir()?
        }
    };
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}
