use std::sync::mpsc::SendError;
use std::{ffi::OsString, fmt, io::Error};

use crate::disk_client::DiskError;
use jammdb::Error as JammError;


pub type AppErrorUnit = AppError<()>;

#[derive(Debug)]
pub enum AppErrorUnitOrT<T: std::fmt::Debug> {
    AppErrorUnit(AppError<()>),
    AppErrorT(AppError<T>)
}

#[derive(Debug)]
pub enum AppError<T: std::fmt::Debug> {
    DiskErrors(DiskError),
    FSErrors(Error),
    InvalidFileName(String),
    InvalidPathToDelete(String),
    ConvertOsStringToStringErr(OsString),
    ConvertPathBufToStr,
    MissingSelectedElelement,
    MultipleErrors(Vec<AppError<T>>),
    LogicalError(String),
    DBError(JammError),
    SendError(SendError<T>)
}

impl <T: std::fmt::Debug> AppError <T> {
    pub fn invalid_f_name(err_message: &str, f_name: &str) -> AppError<T> {
        AppError::InvalidFileName(format!("{} - {}", err_message.to_string(), f_name))
    }
}

impl <T: std::fmt::Debug> fmt::Display for AppError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DiskErrors(disk_error) => write!(f, "{}", disk_error),
            AppError::FSErrors(fs_error) => write!(f, "{}", fs_error),
            AppError::InvalidFileName(error) => write!(f, "{}", error),
            AppError::InvalidPathToDelete(error) => write!(f, "{}", error),
            AppError::ConvertOsStringToStringErr(error) => {
                write!(f, "Convert os string to string error {error:?}")
            }
            AppError::ConvertPathBufToStr => write!(
                f,
                "Invalid conver path buf to string: {}",
                AppError::ConvertPathBufToStr::<()>
            ),
            AppError::MissingSelectedElelement => write!(
                f,
                "Rissing select element: {}",
                AppError::MissingSelectedElelement::<()>
            ),
            AppError::MultipleErrors(app_errors) => {
                write!(f, "Multiple app errors: {app_errors:?}")
            }
            AppError::LogicalError(error) => write!(f, "Logical error: {}", error),
            AppError::DBError(error) => write!(f, "DB error: {}", error),
            AppError::SendError(error) => write!(f, "Channel Send error: {}", error)
        }
    }
}
