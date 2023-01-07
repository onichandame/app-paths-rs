use crate::app_paths::AppPaths;

static DATA_DIR_NAME: &str = "data";

pub struct AppPathsImpl;

impl AppPathsImpl {
    pub fn new() -> Self {
        Self
    }
}

impl AppPaths for AppPathsImpl {
    fn get_data_dir(&self) -> crate::error::Result<std::path::PathBuf> {
        Ok(std::env::current_dir()?.join(DATA_DIR_NAME))
    }
}
