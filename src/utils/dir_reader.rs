use std::{
    collections::{HashMap, HashSet},
    fs::{self},
    io::Error,
    path::PathBuf,
};

use crate::{
    disk_client::{DirItem, DiskClient, ItemResponse},
    error::AppError,
    models::file::{CloudFile, File, LocalFile, NodeType, State},
};
use log::{debug, info};

use super::common::read_f_name;

pub enum ReadingType {
    Local,
    Cloud,
    LocalCloud,
}

#[derive(Debug, Clone)]
pub struct DirReader {
    pub sync_dir_path: String,
    pub disk_client: DiskClient,
}

impl DirReader {
    pub fn read_local_with_cloud(&self, path: &str) -> Result<(File, Vec<File>), AppError> {
        let (_cloud_root_dir, cloud_dir_items) = self.fetch_cloud_dirs(path).map(|d| {
            let items = d
                ._embedded
                .items
                .clone()
                .into_iter()
                .map(|v| (v.name.clone(), v))
                .collect::<HashMap<String, DirItem>>();
            (d, items)
        })?;

        let root_dir = File {
            name: path.to_string(),
            file_type: NodeType::Dir,
            state: State::Synced,
            cloud: Some(CloudFile {}),
            local: Some(LocalFile {
                path: PathBuf::from(self.sync_dir_path.clone() + "/" + path),
            }),
        };

        let dir_items = self.read(path, cloud_dir_items)?;
        Ok((root_dir, dir_items))
    }

    pub fn read_cloud(&self, path: &str) -> Result<(File, Vec<File>), AppError> {
        self.fetch_cloud_dirs(path).map(|d| {
            let item = File {
                name: d.name.clone(),
                file_type: if d.is_dir() {
                    NodeType::Dir
                } else {
                    NodeType::File
                },
                cloud: Some(CloudFile {}),
                local: None,
                state: State::Cloud,
            };

            let child_items = d
                ._embedded
                .items
                .into_iter()
                .map(|item| File {
                    name: item.clone().name,
                    file_type: if item.is_dir() {
                        NodeType::Dir
                    } else {
                        NodeType::File
                    },
                    cloud: Some(CloudFile {}),
                    local: None,
                    state: State::Cloud,
                })
                .collect();

            (item, child_items)
        })
    }

    pub fn read_local(&self, path: &str) -> Result<(File, Vec<File>), AppError> {
        let item = File {
            name: read_f_name(path)?,
            file_type: NodeType::Dir,
            state: State::Local,
            local: Some(LocalFile {
                path: PathBuf::from(path),
            }),
            cloud: None,
        };
        self.read(path, HashMap::new()).map(|items| (item, items))
    }

    fn fetch_cloud_dirs(&self, path: &str) -> Result<ItemResponse, AppError> {
        self.disk_client
            .item(path, None, Some(1000))
            .map_err(AppError::DiskErrors)
    }

    fn read(&self, path: &str, cloud_dir: HashMap<String, DirItem>) -> Result<Vec<File>, AppError> {
        let path_b = PathBuf::from(self.sync_dir_path.clone() + path);
        let dir_entities = fs::read_dir(path_b).map_err(AppError::FSErrors)?;

        let (valid_entites, invalid_entities): (Vec<Result<File, _>>, Vec<Result<_, Error>>) =
            dir_entities
                .map(|entry| {
                    entry.and_then(|e| {
                        let f_name = e.file_name().into_string().unwrap();
                        let f_type = e.file_type()?;
                        // TODO fill
                        let cloud = cloud_dir.get(&f_name).map(|cloud_item| CloudFile {});
                        let local = LocalFile { path: e.path() };
                        let node_type = if f_type.is_file() {
                            NodeType::File
                        } else {
                            NodeType::Dir
                        };

                        Ok(File {
                            name: f_name,
                            file_type: node_type,
                            // TODO: checl file is synced or outdated
                            state: cloud.clone().map(|_| State::Synced).unwrap_or(State::Local),
                            cloud,
                            local: Some(local),
                        })
                    })
                })
                .partition(Result::is_ok);

        let invalid_entites_errors = invalid_entities
            .into_iter()
            .map(Result::unwrap_err)
            .collect::<Vec<Error>>();
        log::info!("Invalid entities {:?}", invalid_entites_errors);

        let local_and_cloud_files = valid_entites
            .into_iter()
            .map(Result::unwrap)
            .collect::<Vec<File>>();

        let local_and_cloud_files_hash = local_and_cloud_files
            .clone()
            .into_iter()
            .map(|f| f.name)
            .collect::<HashSet<String>>();

        let cloud_only_files = cloud_dir
            .into_iter()
            .filter(|(_, v)| !local_and_cloud_files_hash.contains(&v.name))
            .map(|(name, di)| File {
                name,
                file_type: if di.is_dir() {
                    NodeType::Dir
                } else {
                    NodeType::File
                },
                state: State::Cloud,
                cloud: Some(CloudFile {}),
                local: None,
            })
            .collect::<Vec<File>>();

        let mut all_files = [local_and_cloud_files, cloud_only_files].concat();
        all_files.sort_by(|current, next| next.file_type.cmp(&current.file_type));

        Ok(all_files)
    }
}
