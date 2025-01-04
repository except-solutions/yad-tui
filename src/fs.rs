use crate::components::main_screen::next_dir::NextDir;
use crate::components::main_screen::{current_dir::CurrentDir, previous_dir::PreviousDir};
use crate::disk_client::{DirItem, DiskClient, DiskError};
use crate::error::AppError;
use crate::models::file::{CloudFile, File, LocalFile, NodeType, State};
use crate::models::model::Model;
use crate::utils::dir_reader::DirReader;
use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::path::PathBuf;


pub trait ReaderHOF {
    fn from_path(path_buf: &String, dir_reader: DirReader) -> Self;
}

#[derive(Debug)]
pub struct FS {
    pub next_dir: Option<NextDir>,
    pub current_dir: CurrentDir,
    pub previous_dir: PreviousDir,
    pub dir_reader: DirReader
}

impl ReaderHOF for FS {
    fn from_path(path: &String, dir_reader: DirReader) -> Self {
        let current_dir = CurrentDir::from_path(path, dir_reader.clone());
        let previous_dir = PreviousDir::from_path(path, dir_reader.clone());
        let next_dir = None;

        Self::new(previous_dir, current_dir, next_dir, dir_reader)
    }
}

impl FS {
    pub fn new(
        previous_dir: PreviousDir,
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

                self.previous_dir = PreviousDir::from_path(&path, self.dir_reader.clone());
                self.current_dir = CurrentDir::from_path(&path, self.dir_reader.clone())
            }
        }
    }

    pub fn open_previous(&mut self) {
        let path = String::from(self.previous_dir.path.to_str().unwrap());
        self.current_dir = CurrentDir::from_path(&path, self.dir_reader.clone());
        self.previous_dir = PreviousDir::from_path(&path, self.dir_reader.clone());
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
                    self.next_dir = Some(NextDir::from_path(&String::from(path), self.dir_reader.clone()))
                }
            }
        }
    }
}

// pub fn read_dir(path: &str, model: &Model) -> Result<Vec<File>, AppErrors> {
//     let path_b = PathBuf::from(model.config.main.sync_dir_path.clone() + path.clone());
//     // TODO: implement paging
//     let disk_items = model
//         .disk_client
//         .item(path, None, Some(1000))
//         .map(|item| {
//             item._embedded
//                 .items
//                 .into_iter()
//                 .map(|i| (i.name.clone(), i))
//                 .collect::<HashMap<String, DirItem>>()
//         })
//         .map_err(AppErrors::DiskErrors)?;
//     let dir_entities = fs::read_dir(path_b).map_err(AppErrors::FSErrors)?;
// 
//     let (valid_entites, invalid_entities) : (Vec<Result<File, _>>, Vec<_>) = dir_entities
//         .map(|entry| {
//             entry.and_then(|e| {
//                 let f_name = e.file_name().into_string().unwrap();
//                 let f_type = e.file_type()?;
//                 // TODO fill
//                 let cloud = disk_items.get(&f_name).map(|cloud_item| CloudFile {});
//                 let local = LocalFile { path: e.path() };
//                 let node_type = if f_type.is_file() {
//                     NodeType::File
//                 } else {
//                     NodeType::Dir
//                 };
// 
//                 Ok(File {
//                     name: f_name,
//                     file_type: node_type,
//                     state: cloud.clone().map(|_| State::Synced).unwrap_or(State::Local),
//                     cloud,
//                     local: Some(local),
//                 })
//             })
//         })
//         .partition(Result::is_ok);
//     let collected = valid_entites.into_iter().map(Result::unwrap).collect::<Vec<File>>();
//     // TODO sort results, add clouds only files,  prepare errors    
//     Ok(collected)
// }

//pub fn read_dir(path: &str, model: &Model) -> Vec<File> {
//    match fs::read_dir(path) {
//        Ok(entries) => {
//            let (results, errors): (Vec<_>, Vec<_>) = entries
//                .map(|dir_entry| match dir_entry {
//                    Ok(dir_entry) => match dir_entry.file_type() {
//                        Ok(file_type) => Ok(File {
//                            name: dir_entry.file_name().into_string().unwrap(),
//                            file_type: if file_type.is_file() {
//                                NodeType::File
//                            } else {
//                                NodeType::Dir
//                            },
//                            path: dir_entry.path(),
//                        }),
//                        Err(err) => Err(format!("Unexpected file type. Original exc: {err}")),
//                    },
//                    Err(err) => Err(format!(
//                        "Unexpected entity in file system. Original exc: {err}"
//                    )),
//                })
//                .partition(Result::is_ok);
//
//            let mut items: Vec<File> = results.into_iter().map(Result::unwrap).collect();
//            items.sort_by(|current, next| next.file_type.cmp(&current.file_type));
//
//            let _errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
//            items
//        }
//        Err(err) => {
//            panic!("Can`t read `path`. Original exc: {err}", err = err)
//        }
//    }
//}
