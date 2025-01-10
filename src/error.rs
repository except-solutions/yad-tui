use std::io::Error;

use crate::disk_client::DiskError;

#[derive(Debug)]
pub enum AppError {
    DiskErrors(DiskError),
    FSErrors(Error),
}
