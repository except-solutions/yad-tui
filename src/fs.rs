use crate::components::main_screen::next_dir::NextDir;
use crate::components::main_screen::{current_dir::CurrentDir, previous_dir::PreviousDir};
use crate::error::AppError;
use crate::models::file::{NodeType, State};
use crate::utils::common::path_buf_to_string;
use crate::utils::dir_reader::DirReader;
use std::path::PathBuf;

pub trait ReaderHOF {
    fn from_path(path: PathBuf, dir_reader: DirReader) -> Self;
}

#[derive(Debug, Clone)]
pub struct FS {
    pub previous_dir: Option<PreviousDir>,
    pub current_dir: CurrentDir,
    pub next_dir: Option<NextDir>,
    pub dir_reader: DirReader,
}

impl FS {
    pub fn create(dir_reader: DirReader) -> Self {
        let path = PathBuf::from("/");
        let (item, items) = dir_reader.read_dir(path_buf_to_string(path.clone()).unwrap(), State::Synced).unwrap();
        let current_dir = CurrentDir::new(path.clone(), item, items);
        let previous_dir = None;

        let next_dir = current_dir.items.first().and_then(|item| {
            if item.state.in_cloud() {
                let mut next_path = path.clone();
                next_path.push(item.name.clone());
                let (item, items) = dir_reader
                     .read_dir(path_buf_to_string(next_path.clone()).unwrap(), item.state.clone()).unwrap();
                Some(NextDir::new(next_path, item, items))
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

    pub fn open_selected(&mut self) -> Result<(), AppError> {
        let selected = &self.current_dir.selected_file()?;

        if selected.is_dir() {
            let path = self.current_dir.path.join(selected.name.clone());
            self.previous_dir = Some(PreviousDir::new(
                self.current_dir.path.clone(),
                self.current_dir.item.clone(),
                self.current_dir.items.clone(),
            ));

            let (item, items) = self
                .dir_reader
                .read_dir(path_buf_to_string(path.clone())?, selected.state.clone())?;
            self.current_dir = CurrentDir::new(path.clone(), item, items);
            self.set_next_dir_from_current();
        }

        Ok(())
    }

    pub fn open_previous(&mut self) -> Result<(), AppError> {
        if let Some(previous_dir) = &self.previous_dir {

            let (current_item, current_dir_items) = self
                .dir_reader
                .read_dir(path_buf_to_string(previous_dir.path.clone())?, previous_dir.item.state.clone())?;

            self.current_dir = CurrentDir::new(previous_dir.path.clone(), current_item, current_dir_items);

            if let Some(prev_path) = previous_dir.path.parent() {
                let prev_path_buf = prev_path.to_path_buf();
                let (item, items) = self.dir_reader.read_dir(path_buf_to_string(prev_path_buf.clone())?, previous_dir.item.state.clone())?;
                self.previous_dir = Some(PreviousDir::new(prev_path_buf, item, items));
            } else {
                self.previous_dir = None;
            }
        }

        Ok(())
    }

    fn set_next_dir_from_current(&mut self) -> Result<(), AppError> {
        let selected = self
            .current_dir
            .selected_file()?;

            if selected.file_type == NodeType::Dir {
                let mut next_path_buf = PathBuf::new();
                next_path_buf.push(&self.current_dir.path);
                next_path_buf.push(&selected.name);
                let (item, items) = self
                     .dir_reader
                     .read_dir(path_buf_to_string(next_path_buf.clone())?, selected.state.clone())?;
                self.next_dir = Some(NextDir::new(next_path_buf, item, items));
            } else {
                self.next_dir = None
            }
        Ok(())
    }
}
