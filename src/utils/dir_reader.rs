use std::{
    collections::{HashMap, HashSet},
    fs::{self, ReadDir},
    io::Error,
    path::PathBuf,
    sync::Arc,
};

use crate::{
    disk_client::{DirItem, DiskClient, DiskClientT, ItemResponse},
    error::AppError,
    models::file::{CloudFile, File, LocalFile, NodeType, State},
};

use super::common::read_f_name;


#[derive(Debug, Clone)]
pub struct DirReader<T: DiskClientT> {
    pub sync_dir_path: String,
    pub disk_client: Arc<T>,
}

impl<T: DiskClientT> DirReader<T> {
    pub fn read_dir(&self, path: String) -> Result<(File, Vec<File>), AppError> {
        let path_buf = PathBuf::from(self.sync_dir_path.clone() + "/" + path.as_str());

        let cloud_dir = self.fetch_cloud_dirs(path.clone());

        let (cloud_root_dir, cloud_dir_items) = self
            .fetch_cloud_dirs(path.clone())
            .map(|d| {
                let items = d
                    ._embedded
                    .items
                    .clone()
                    .into_iter()
                    .map(|v| (v.name.clone(), v))
                    .collect::<HashMap<String, DirItem>>();
                (Some(d), items)
            })
            .unwrap_or((None, HashMap::new()));

        let local_dir = fs::read_dir(path_buf.clone()).map_err(AppError::FSErrors);
        let local_dir_exists = local_dir.is_ok();

        let root_dir_f = |state: State| {
            Ok(File {
                name: read_f_name(path.clone())?,
                file_type: NodeType::Dir,
                state,
                cloud: cloud_root_dir.map(|cd| CloudFile::new(cd.path)),
                local: if local_dir_exists {
                    Some(LocalFile { path: path_buf })
                } else {
                    None
                },
            })
        };

        match (cloud_dir, local_dir) {
            (Ok(_), Ok(local)) => Ok((
                root_dir_f(State::Synced)?,
                self.read(local, cloud_dir_items)?,
            )),
            (Ok(_), Err(_)) => {
                let dir_items = cloud_dir_items
                    .into_values()
                    .map(|item| File {
                        name: item.clone().name,
                        file_type: if item.is_dir() {
                            NodeType::Dir
                        } else {
                            NodeType::File
                        },
                        cloud: Some(CloudFile::new(path.clone())),
                        local: None,
                        state: State::Cloud,
                    })
                    .collect::<Vec<File>>();
                Ok((root_dir_f(State::Cloud)?, dir_items))
            }
            (Err(_), Ok(local)) => {
                Ok((root_dir_f(State::Local)?, self.read(local, HashMap::new())?))
            }
            (Err(cloud_error), Err(local_error)) => {
                Err(AppError::MultipleErrors(vec![cloud_error, local_error]))
            }
        }
    }

    fn fetch_cloud_dirs(&self, path: String) -> Result<ItemResponse, AppError> {
        self.disk_client
            .item(path.as_str(), None, Some(1000))
            .map_err(AppError::DiskErrors)
    }

    fn read(
        &self,
        local_dir: ReadDir,
        cloud_dir: HashMap<String, DirItem>,
    ) -> Result<Vec<File>, AppError> {
        let (valid_entites, invalid_entities): (Vec<Result<File, _>>, Vec<Result<_, Error>>) =
            local_dir
                .map(|entry| {
                    entry.and_then(|e| {
                        let f_name = e.file_name().into_string().unwrap();
                        let f_type = e.file_type()?;
                        // TODO fill
                        let cloud = cloud_dir
                            .get(&f_name)
                            .map(|cloud_item| CloudFile::new(cloud_item.path.clone()));
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
                cloud: Some(CloudFile::new(di.path)),
                local: None,
            })
            .collect::<Vec<File>>();

        let mut all_files = [local_and_cloud_files, cloud_only_files].concat();
        all_files.sort_by(|current, next| next.file_type.cmp(&current.file_type));

        Ok(all_files)
    }
}
