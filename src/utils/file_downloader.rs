use std::sync::Arc;

use crate::{config::Config, utils::dir_reader::DirReader};
use crate::components::main_screen::next_dir::NextDir;
use crate::components::main_screen::{current_dir::CurrentDir, previous_dir::PreviousDir};
use crate::error::AppError;
use crate::models::file::{CloudFile, File, NodeType, State};
use crate::utils::common::path_buf_to_string;
use crate::utils::progress_file_reader::ProgressFileReader;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use log;


#[derive(Debug, Clone)]
pub struct FileDownloader {
    pub config: Arc<Config>,
    pub dir_reader: Arc<DirReader>
}

impl FileDownloader {


    pub fn download(&self, sender: Sender<usize>, file_to_download: File) -> Result<(), AppError> {
        if let Some(cloud_file) = file_to_download.cloud {
            let unix_time = SystemTime::now().duration_since(UNIX_EPOCH);
            let file_path = &cloud_file.path[5..];
            let temp_dir_path = format!("{}/tmp/{}_{:?}", self.config.main.cache_dir_path, file_path, unix_time.unwrap().as_secs());
            let temp_file_name = match file_to_download.file_type {
                NodeType::File => file_to_download.name,
                NodeType::Dir  => format!("{}.zip", file_to_download.name)
            };
            let temp_full_path = format!("{}/{}", temp_dir_path, temp_file_name);
            fs::create_dir_all(temp_dir_path).unwrap();
            let disk_client = self.dir_reader.disk_client.clone();
            let sync_dir = self.dir_reader.sync_dir_path.clone();

            thread::spawn(move || {
                let disk_file_reader = disk_client
                    .file_reader(cloud_file.path.clone())
                    .map_err(AppError::DiskErrors)?;

                let mut wrapper = ProgressFileReader {
                    bytes_readed: 0,
                    it: disk_file_reader,
                    sender,
                };

                let mut temp_file =
                    fs::File::create(temp_full_path.clone()).map_err(AppError::FSErrors)?;

                std::io::copy(&mut wrapper, &mut temp_file).map_err(AppError::FSErrors)?;

                let local_path = format!("{}{}", sync_dir, &cloud_file.path[5..]);

                if let NodeType::Dir = file_to_download.file_type {

                   // TODO add wrapper for fs actions ?  
                    let dir_zipped_archive = fs::File::open(temp_full_path.clone()).map_err(AppError::FSErrors)?;
                    if fs::metadata(local_path.clone()).is_ok() {

                        fs::remove_dir_all(local_path.clone()).map_err(AppError::FSErrors)?;
                    }
                    //TODO map zip error in apperror

                    zip_extract::extract(Cursor::new(dir_zipped_archive).get_ref(), &PathBuf::from(local_path), true).unwrap();
        
                } else {
                    fs::rename(temp_full_path.clone(), local_path).unwrap();
                };
                
                Result::<(), AppError>::Ok(())
            });
        };

        Ok(())

    }
}

