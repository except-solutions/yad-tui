use std::{collections::HashMap, fs::{self, DirEntry}, io::Error, path::PathBuf};

use crate::{disk_client::{DirItem, DiskClient}, error::AppError, models::file::{CloudFile, File, LocalFile, NodeType, State}};


pub enum ReadingType {
    Local,
    Cloud,
    LocalCloud
}

#[derive(Debug, Clone)]
pub struct DirReader {
    pub sync_dir_path: String,
    pub disk_client: DiskClient
}

impl DirReader {

    pub fn read(&self, path: &str, reading_type: ReadingType) -> Result<Vec<File>, AppError> {
        let path_b = PathBuf::from(self.sync_dir_path.clone() + path);
        // TODO: implement paging
        
        let x = match reading_type {
            ReadingType::Local => {

                let disk_items_stub = &HashMap::new();
                self.read_local(disk_items_stub)
            },
            ReadingType::Cloud => {
                let disk_items = &self.disk_client
                    .item(path, None, Some(1000))
                    .map(|item| {
                        item._embedded
                            .items
                            .into_iter()
                            .map(|i| (i.name.clone(), i))
                            .collect::<HashMap<String, DirItem>>()
                    })
                    .map_err(AppError::DiskErrors)?;
               (disk_items, Self::local_mapper) 
            },
            ReadingType::LocalCloud => {
                let disk_items = &self.disk_client
                    .item(path, None, Some(1000))
                    .map(|item| {
                        item._embedded
                            .items
                            .into_iter()
                            .map(|i| (i.name.clone(), i))
                            .collect::<HashMap<String, DirItem>>()
                    })
                    .map_err(AppError::DiskErrors)?;


                    self.read_local(&disk_items)

            }

        };
        let disk_items = &self.disk_client
            .item(path, None, Some(1000))
            .map(|item| {
                item._embedded
                    .items
                    .into_iter()
                    .map(|i| (i.name.clone(), i))
                    .collect::<HashMap<String, DirItem>>()
            })
            .map_err(AppError::DiskErrors)?;
        let dir_entities = fs::read_dir(path_b).map_err(AppError::FSErrors)?;

        let (valid_entites, invalid_entities) : (Vec<Result<File, _>>, Vec<_>) = dir_entities
            .map(|entry| {
                entry.and_then(|e| {
                    let f_name = e.file_name().into_string().unwrap();
                    let f_type = e.file_type()?;
                    // TODO fill
                    let cloud = disk_items.get(&f_name).map(|cloud_item| CloudFile {});
                    let local = LocalFile { path: e.path() };
                    let node_type = if f_type.is_file() {
                        NodeType::File
                    } else {
                        NodeType::Dir
                    };

                    Ok(File {
                        name: f_name,
                        file_type: node_type,
                        state: cloud.clone().map(|_| State::Synced).unwrap_or(State::Local),
                        cloud,
                        local: Some(local),
                    })
                })
            })
            .partition(Result::is_ok);
        let collected = valid_entites.into_iter().map(Result::unwrap).collect::<Vec<File>>();
        // TODO sort results, add clouds only files,  prepare errors    
        Ok(collected)
    }

    fn read_local(&self, disk_items: &HashMap<String, DirItem>) {

    }



//    fn local_mapper(
//        &self,
//        disk_items: HashMap<String, DirItem>
//    ) -> Result<impl Fn(DirEntry) -> Result<File, Error>, AppError> {
//        
//        Ok(move |e: DirEntry| { 
//            let f_name = e.file_name().into_string().unwrap();
//            let f_type = e.file_type()?;
//            // TODO fill
//            let cloud = disk_items.get(&f_name).map(|cloud_item| CloudFile {});
//            let local = LocalFile { path: e.path() };
//            let node_type = if f_type.is_file() {
//                NodeType::File
//            } else {
//                NodeType::Dir
//            };
//            
//            Ok(File {
//                name: f_name,
//                file_type: node_type,
//                state: cloud.clone().map(|_| State::Synced).unwrap_or(State::Local),
//                cloud: cloud.clone(),
//                local: Some(local),
//            })
//        })
//    }

    // fn cloud_mapper(&self, disk_items: HashMap<String, DirItem>) {
    //     &self.disk_client
    //         .item(path, None, Some(1000))
    //         .map(|item| {
    //             item._embedded
    //                 .items
    //                 .into_iter()
    //                 .map(|i| (i.name.clone(), i))
    //                 .collect::<HashMap<String, DirItem>>()
    //         })
    //         .map_err(AppError::DiskErrors)?;

    // }
}
