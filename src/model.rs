use crate::{config::Config, disk_client::DiskClient, meta_db::Meta};
use jammdb::DB;
use std::{fmt, path::PathBuf};

#[derive(Clone, Debug, Ord, Eq, PartialOrd, PartialEq)]
pub enum NodeType {
    File,
    Dir,
}

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub active: bool,
    pub file_type: NodeType,
}

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
    pub current_dir: Vec<File>,
    pub sub_dir: Vec<File>,
    pub active_file_row_index: i32,
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
            .field("current_dir", &self.current_dir)
            .field("sub_dir", &self.sub_dir)
            .field("active_file_row_index", &self.active_file_row_index)
            .field("popup", &self.popup)
            .field("config", &self.config)
            .field("config_path", &self.config_path)
            .field("meta", &self.meta)
            .field("disk_client", &self.disk_client)
            .finish()
    }
}

impl Model {
    pub fn set_active_file(&mut self, new_pos_index: i32) -> Result<(), String> {
        if new_pos_index < 0 || new_pos_index >= self.current_dir.len() as i32 {
            Err::<(), String>(format!("Inavlid index {new_pos_index}"))
        } else {
            self.active_file_row_index = new_pos_index;
            self.current_dir = self
                .current_dir
                .iter()
                .enumerate()
                .map(|(i, file)| File {
                    active: i == new_pos_index as usize,
                    ..file.clone()
                })
                .collect();

            Ok(())
        }
    }
}
