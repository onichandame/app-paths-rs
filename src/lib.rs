pub use app_paths::AppPaths;
use error::Result;

#[cfg(target_os = "android")]
mod android;
mod app_paths;
mod error;
mod fallback;
#[cfg(target_os = "windows")]
mod windows;

/// main entrypoint of this lib. returns an instance of the platform-specific [AppPaths][AppPaths]
/// implementation
pub fn init() -> Result<impl AppPaths> {
    #[cfg(target_os = "android")]
    {
        android::AppPathsImpl::create()
    }
    #[cfg(not(any(target_os = "android")))]
    {
        Ok(fallback::AppPathsImpl::new())
    }
}
