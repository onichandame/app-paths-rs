static DATA_DIR_NAME: &str = "data";

pub fn get_data_dir() -> crate::error::Result<std::path::PathBuf> {
    Ok(std::env::current_dir()?.join(DATA_DIR_NAME))
}
