use crate::components::main_screen::next_dir::NextDir;
use crate::components::main_screen::{current_dir::CurrentDir, previous_dir::PreviousDir};
use crate::models::file::{NodeType, State};
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
        let current_dir = CurrentDir::from_path(path.clone(), dir_reader.clone());
        let previous_dir = None;

        let next_dir = current_dir.items.first().and_then(|item| {
            if item.state.in_cloud() {
                let mut next_path = path.clone();
                next_path.push(item.name.clone());
                Some(NextDir::from_path(next_path, dir_reader.clone()))
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
                let path = self.current_dir.path.join(selected.name.clone());
                self.previous_dir = Some(PreviousDir::new(
                    self.current_dir.path.clone(),
                    self.current_dir.item.clone(),
                    self.current_dir.items.clone(),
                ));

                match selected.state {
                    State::Cloud => {
                        let (item, items) = self
                            .dir_reader
                            .clone()
                            .read_cloud(path.as_os_str().to_str().unwrap())
                            .unwrap();

                        self.current_dir =
                            CurrentDir::new(self.current_dir.path.clone(), item, items);
                    }
                    State::Local => {
                        let (item, items) = self
                            .dir_reader
                            .clone()
                            .read_local(path.as_os_str().to_str().unwrap())
                            .unwrap();

                        self.current_dir =
                            CurrentDir::new(self.current_dir.path.clone(), item, items);
                    }
                    State::Synced | State::Syncing => {
                        let (item, items) = self
                            .dir_reader
                            .clone()
                            .read_local_with_cloud(path.as_os_str().to_str().unwrap())
                            .unwrap();

                        self.current_dir =
                            CurrentDir::new(self.current_dir.path.clone(), item, items);
                    }
                };
            }
        }
    }

    pub fn open_previous(&mut self) {
        if let Some(previous_dir) = &self.previous_dir {
            self.current_dir =
                CurrentDir::from_path(previous_dir.path.clone(), self.dir_reader.clone());

            self.previous_dir = previous_dir
                .path
                .parent()
                .map(|b| PreviousDir::from_path(b.to_path_buf(), self.dir_reader.clone()));
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

                self.next_dir = Some(NextDir::from_path(next_path_buf, self.dir_reader.clone()))
            }
        }
    }
}
