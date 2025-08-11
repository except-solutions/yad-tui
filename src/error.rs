use std::{ffi::OsString, io::Error};

use crate::disk_client::DiskError;

#[derive(Debug)]
pub enum AppError {
    DiskErrors(DiskError),
    FSErrors(Error),
    InvalidFileName(String),
    InvalidPathToDelete(String),
    ConvertOsStringToStringErr(OsString),
    ConvertPathBufToStr,
    MissingSelectedElelement,
    MultipleErrors(Vec<AppError>),
    LogicalError(String),
}

impl AppError {
    pub fn invalid_f_name(err_message: &str, f_name: &str) -> AppError {
        AppError::InvalidFileName(format!("{} - {}", err_message.to_string(), f_name))
    }
}
