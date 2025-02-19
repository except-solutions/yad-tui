use crate::channels::Channels;
use crate::components::main_screen::top_bar::TopBar;
use crate::fs::FS;
use crate::{config::Config, disk_client::DiskClient};
use jammdb::DB;
use std::{fmt, path::PathBuf};

#[derive(Debug, Clone)]
pub enum Popup {
    Config,
    LoginForm {
        code_input: String,
        error_message: Option<String>,
    },
}

pub struct Model {
    pub top_bar: Option<TopBar>,
    pub fs: FS,
    pub popup: Option<Popup>,
    pub config: Config,
    pub config_path: PathBuf,
    pub meta_db: DB,
    pub disk_client: DiskClient,
    pub channels: Channels
}

impl fmt::Debug for Model {
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

impl Model {
    pub fn is_authenticated(&self) -> bool {
        self.disk_client.token.is_some()
    }
}
