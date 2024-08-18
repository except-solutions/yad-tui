use crate::config::Config;
use std::path::PathBuf;

#[derive(Clone, Debug)]
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

#[derive(Debug, Default, Clone)]
pub struct Popup {
    pub show_config: bool,
}

#[derive(Debug, Default)]
pub struct Model {
    pub previous_dir: Vec<File>,
    pub current_dir: Vec<File>,
    pub sub_dir: Vec<File>,
    pub active_file_row_index: i32,
    pub popup: Popup,
    pub config: Config,
    pub config_path: PathBuf,
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
