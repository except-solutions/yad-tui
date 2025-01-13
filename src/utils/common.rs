use std::path::PathBuf;

use crate::error::AppError;

pub fn bytes_to_gbytes(bytes: u64) -> f64 {
    bytes as f64 / 8.0_f64.powf(10.0)
}

pub fn read_f_name(path: &str) -> Result<String, AppError> {
    let dir_path = PathBuf::from(path);

    let binding = dir_path
        .file_name()
        .ok_or(AppError::invalid_f_name("Can't read file name", path))?
        .to_os_string();
    binding
        .into_string()
        .map_err(AppError::ConvertOsStringToStringErr)
}
