use std::path::PathBuf;

use windows::Storage::AppDataPaths;

use crate::{error::Result, fallback};

pub fn get_data_dir() -> Result<PathBuf> {
    if let Ok(paths) = AppDataPaths::GetDefault() {
        Ok(paths.LocalAppData()?.to_string().into())
    } else {
        Ok(fallback::get_data_dir()?)
    }
}
