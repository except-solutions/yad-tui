use std::sync::Arc;

use crate::{
    config::Config,
    disk_client::DiskClientT,
    error::{AppErrorUnit, AppErrorUnitOrT},
    utils::dir_reader::DirReader,
};

#[derive(Debug, Clone)]
pub struct FileUploader<DC: DiskClientT> {
    pub config: Arc<Config>,
    pub disk_client: Arc<DC>,
}

impl<DC: DiskClientT> FileUploader<DC> {
    pub fn upload(&self) -> Result<(), AppErrorUnit> {
        // TODO rewrite tests first;
        //
        Ok(())
    }
}
