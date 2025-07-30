use std::{fs, path::PathBuf};

use crate::error::AppError;

pub fn bytes_to_gbytes(bytes: u64) -> f64 {
    bytes as f64 / 8.0_f64.powf(10.0)
}

pub fn read_f_name(path: String) -> Result<String, AppError> {
    if path == "/" {
        Ok("/".to_string())
    } else {
        let dir_path = PathBuf::from(path.clone());

        let binding = dir_path
            .file_name()
            .ok_or(AppError::invalid_f_name(
                "Can't read file name",
                path.as_str(),
            ))?
            .to_os_string();
        binding
            .into_string()
            .map_err(AppError::ConvertOsStringToStringErr)
    }
}

pub fn path_buf_to_string(path: PathBuf) -> Result<String, AppError> {
    path.into_os_string()
        .into_string()
        .map_err(AppError::ConvertOsStringToStringErr)
}
