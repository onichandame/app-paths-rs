use std::path::PathBuf;

use windows::Storage::AppDataPaths;

use crate::{error::Result, fallback, Error};

pub fn get_data_dir() -> Result<PathBuf> {
    if let Ok(dir) = (|| {
        let paths = AppDataPaths::GetDefault()?;
        Ok::<String, Error>(paths.LocalAppData()?.to_string())
    })() {
        Ok(dir.into())
    } else {
        Ok(fallback::get_data_dir()?)
    }
}
