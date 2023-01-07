use std::path::PathBuf;

use crate::error::Result;

/// A collection of methods managing paths for an application in a cross-platform
/// manner
pub trait AppPaths {
    /// Get the path to the directory where all the app data should be stored
    fn get_data_dir(&self) -> Result<PathBuf>;
}
