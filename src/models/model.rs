use crate::components::main_screen::top_bar::TopBar;
use crate::disk_client::DiskClientT;
use crate::fs::FS;
use crate::{config::Config, disk_client::DiskClient};
use jammdb::DB;
use std::sync::Arc;
use std::{fmt, path::PathBuf};

#[derive(Debug, Clone)]
pub enum Popup {
    Config,
    LoginForm {
        code_input: String,
        error_message: Option<String>,
    },
}

pub struct Model<T: DiskClientT + Send + Sync + 'static> {
    pub is_auth: bool,
    pub top_bar: Option<TopBar>,
    pub fs: FS<T>,
    pub popup: Option<Popup>,
    pub config: Config,
    pub config_path: PathBuf,
    pub meta_db: DB,
    pub disk_client: Arc<T>,
}

impl<T> fmt::Debug for Model<T>
where
    T: DiskClientT + Send + Sync,
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Model")
            .field("fs", &self.fs)
            .field("popup", &self.popup)
            .field("config", &self.config)
            .field("config_path", &self.config_path)
            .field("disk_client", &self.disk_client)
            .finish()
    }
}

impl<T: DiskClientT + Sync + Send + 'static> Model<T> {
    pub fn is_authenticated(&self) -> bool {
        self.is_auth
    }
}
