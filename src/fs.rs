use crate::components::main_screen::next_dir::NextDir;
use crate::components::main_screen::{current_dir::CurrentDir, previous_dir::PreviousDir};
use crate::models::file::{File, NodeType};
use std::fs;
use std::path::PathBuf;

pub trait ReaderHOF {
    fn from_path(path_buf: &String, fs_reader: fn(&PathBuf) -> Vec<File>) -> Self;
}

#[derive(Debug)]
pub struct FS {
    pub next_dir: Option<NextDir>,
    pub current_dir: CurrentDir,
    pub previous_dir: PreviousDir,
    pub reader_func: fn(&PathBuf) -> Vec<File>,
}

impl ReaderHOF for FS {
    fn from_path(path: &String, fs_reader: fn(&PathBuf) -> Vec<File>) -> Self {
        let current_dir = CurrentDir::from_path(path, fs_reader);
        let previous_dir = PreviousDir::from_path(path, fs_reader);
        let next_dir = None;

        Self::new(previous_dir, current_dir, next_dir, fs_reader)
    }
}

impl FS {
    pub fn new(
        previous_dir: PreviousDir,
        current_dir: CurrentDir,
        next_dir: Option<NextDir>,
        reader_func: fn(&PathBuf) -> Vec<File>,
    ) -> Self {
        Self {
            next_dir,
            current_dir,
            previous_dir,
            reader_func,
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
                let path = String::from(selected.path.to_str().unwrap());

                self.previous_dir = PreviousDir::from_path(&path, self.reader_func);
                self.current_dir = CurrentDir::from_path(&path, self.reader_func)
            }
        }
    }

    pub fn open_previous(&mut self) {
        let path = String::from(self.previous_dir.path.to_str().unwrap());
        self.current_dir = CurrentDir::from_path(&path, self.reader_func);
        self.previous_dir = PreviousDir::from_path(&path, self.reader_func)
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
                    self.next_dir = Some(NextDir::from_path(&String::from(path), self.reader_func))
                }
            }
        }
    }
}

pub fn read_dir(path: &PathBuf) -> Vec<File> {
    match fs::read_dir(path) {
        Ok(entries) => {
            let (results, errors): (Vec<_>, Vec<_>) = entries
                .map(|dir_entry| match dir_entry {
                    Ok(dir_entry) => match dir_entry.file_type() {
                        Ok(file_type) => Ok(File {
                            name: dir_entry.file_name().into_string().unwrap(),
                            file_type: if file_type.is_file() {
                                NodeType::File
                            } else {
                                NodeType::Dir
                            },
                            path: dir_entry.path(),
                        }),
                        Err(err) => Err(format!("Unexpected file type. Original exc: {err}")),
                    },
                    Err(err) => Err(format!(
                        "Unexpected entity in file system. Original exc: {err}"
                    )),
                })
                .partition(Result::is_ok);

            let mut items: Vec<File> = results.into_iter().map(Result::unwrap).collect();
            items.sort_by(|current, next| next.file_type.cmp(&current.file_type));

            let _errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
            items
        }
        Err(err) => {
            panic!("Can`t read `path`. Original exc: {err}", err = err)
        }
    }
}
