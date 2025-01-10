use crate::components::main_screen::next_dir::NextDir;
use crate::components::main_screen::{current_dir::CurrentDir, previous_dir::PreviousDir};
use crate::models::file::NodeType;
use crate::utils::dir_reader::DirReader;
use std::path::PathBuf;

pub trait ReaderHOF {
    fn from_path(path: &String, dir_reader: DirReader) -> Self;
}

#[derive(Debug, Clone)]
pub struct FS {
    pub previous_dir: Option<PreviousDir>,
    pub current_dir: CurrentDir,
    pub next_dir: Option<NextDir>,
    pub dir_reader: DirReader,
}

impl ReaderHOF for FS {
    fn from_path(path: &String, dir_reader: DirReader) -> Self {
        let current_dir = CurrentDir::from_path(path, dir_reader.clone());
        let previous_dir = if path == "/" {
            None
        } else {
            Some(PreviousDir::from_path(path, dir_reader.clone()))
        };
        let next_dir = current_dir.items.first().and_then(|item| {
            if item.state.in_cloud() {
                Some(NextDir::from_path(
                    &format!("{}{}", path, &item.name),
                    dir_reader.clone(),
                ))
            } else {
                None
            }
        });
        Self::new(previous_dir, current_dir, next_dir, dir_reader)
    }
}

impl FS {
    pub fn new(
        previous_dir: Option<PreviousDir>,
        current_dir: CurrentDir,
        next_dir: Option<NextDir>,
        dir_reader: DirReader,
    ) -> Self {
        Self {
            next_dir,
            current_dir,
            previous_dir,
            dir_reader,
        }
    }

    pub fn select_next_element_for_next_dir(&mut self) {
        self.current_dir.state.select_next();
        self.set_next_dir_from_current();
    }

    pub fn select_previous_element_for_next_dir(&mut self) {
        self.current_dir.state.select_previous();
        self.set_next_dir_from_current();
    }

    pub fn open_selected(&mut self) {
        let selected = self
            .current_dir
            .items
            .get(self.current_dir.state.selected().unwrap());

        if let Some(selected) = selected {
            if selected.is_dir() {
                let path = String::from(selected.local.clone().unwrap().path.to_str().unwrap());

                self.previous_dir = Some(PreviousDir::from_path(&path, self.dir_reader.clone()));
                self.current_dir = CurrentDir::from_path(&path, self.dir_reader.clone())
            }
        }
    }

    pub fn open_previous(&mut self) {

        if let Some(previous_dir) = &self.previous_dir {
            let path = String::from(previous_dir.path.to_str().unwrap());
            self.current_dir = CurrentDir::from_path(&path, self.dir_reader.clone());
            self.previous_dir = Some(PreviousDir::from_path(&path, self.dir_reader.clone()));
        }
    }

    fn set_next_dir_from_current(&mut self) {
        let selected = self
            .current_dir
            .items
            .get(self.current_dir.state.selected().unwrap());

        if let Some(selected) = selected {
            if selected.file_type == NodeType::Dir {
                let mut next_path_buf = PathBuf::new();
                next_path_buf.push(&self.current_dir.path);
                next_path_buf.push(&selected.name);

                if let Some(path) = next_path_buf.to_str() {
                    self.next_dir = Some(NextDir::from_path(
                        &String::from(path),
                        self.dir_reader.clone(),
                    ))
                }
            }
        }
    }
}
