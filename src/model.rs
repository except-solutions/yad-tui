use crate::components::main_screen_current_dirs::CurrentDirs;
use crate::structs::file::File;
use crate::{config::Config, disk_client::DiskClient, meta_db::Meta};
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
    pub previous_dir: Vec<File>,
    pub current_dirs: CurrentDirs,
    pub sub_dir: Vec<File>,
    pub popup: Option<Popup>,
    pub config: Config,
    pub config_path: PathBuf,
    pub meta: Meta,
    pub meta_db: DB,
    pub disk_client: DiskClient,
}

impl fmt::Debug for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Model")
            .field("previos_dir", &self.previous_dir)
            .field("current_dir", &self.current_dirs)
            .field("sub_dir", &self.sub_dir)
            .field("popup", &self.popup)
            .field("config", &self.config)
            .field("config_path", &self.config_path)
            .field("meta", &self.meta)
            .field("disk_client", &self.disk_client)
            .finish()
    }
}
