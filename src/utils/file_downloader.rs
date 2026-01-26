use crate::disk_client::DiskClientT;
use crate::error::AppError;
use std::sync::Arc;

use crate::error::AppErrorUnit;
use crate::models::file::{File, NodeType};
use crate::utils::progress_file_reader::ProgressFileReader;
use crate::{config::Config, utils::dir_reader::DirReader};
use log;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct FileDownloader<T: DiskClientT> {
    pub config: Arc<Config>,
    pub dir_reader: Arc<DirReader<T>>,
    pub disk_client: Arc<T>,
}

impl<T: DiskClientT> FileDownloader<T> {
    pub fn download(
        &self,
        sender: Sender<usize>,
        file_to_download: File,
    ) -> Result<thread::JoinHandle<Result<(), AppErrorUnit>>, AppErrorUnit> {
        if let Some(cloud_file) = file_to_download.cloud.clone() {
            let unix_time = SystemTime::now().duration_since(UNIX_EPOCH);
            let file_path = &cloud_file.path;
            let temp_dir_path = format!(
                "{}/tmp/{}_{:?}",
                self.config.main.cache_dir_path,
                file_path,
                unix_time.unwrap().as_secs()
            );
            let temp_file_name = match file_to_download.file_type {
                NodeType::File => file_to_download.name.clone(),
                NodeType::Dir => format!("{}.zip", file_to_download.name.clone()),
            };
            let temp_full_path = format!("{}/{}", temp_dir_path, temp_file_name);
            fs::create_dir_all(temp_dir_path).unwrap();
            let disk_client = self.disk_client.clone();
            let sync_dir = self.dir_reader.sync_dir_path.clone();
            let local_path = format!("{}{}", sync_dir, &cloud_file.path);

            log::info!(
                "Try download file: {:?}, by path on disk {}",
                &file_to_download,
                &local_path
            );

            Ok(thread::spawn(move || {
                log::info!("Try download file from cloud by path {}", &cloud_file.path);

                let disk_file_reader = disk_client
                    .file_reader(cloud_file.path.clone())
                    .map_err(AppError::DiskErrors::<()>)?;

                let mut wrapper = ProgressFileReader {
                    bytes_readed: 0,
                    it: disk_file_reader,
                    sender,
                };

                let mut temp_file =
                    fs::File::create(temp_full_path.clone()).map_err(AppError::FSErrors)?;

                std::io::copy(&mut wrapper, &mut temp_file).map_err(AppError::FSErrors)?;

                if let NodeType::Dir = file_to_download.file_type {
                    log::info!(
                        "Open downlaoded dir: {:?} archive, by path: {}",
                        &file_to_download,
                        &temp_full_path
                    );

                    let dir_zipped_archive =
                        fs::File::open(temp_full_path.clone()).map_err(AppError::FSErrors)?;
                    if fs::metadata(local_path.clone()).is_ok() {
                        log::info!(
                            "Local file on path {} already exists, try remove him for replace",
                            &local_path
                        );

                        fs::remove_dir_all(local_path.clone()).map_err(AppError::FSErrors)?;

                        log::info!("Success remove old local file: {}", &local_path)
                    }
                    log::info!("Try extract dir archive on path: {}", &local_path);
                    // TODO map zip error to app error
                    zip_extract::extract(
                        Cursor::new(dir_zipped_archive).get_ref(),
                        &PathBuf::from(&local_path),
                        true,
                    )
                    .unwrap();

                    log::info!("Success extract dir archive on path: {}", &local_path);
                } else {
                    log::info!(
                        "Move downloaded file from tmp: {} to local disk path: {}",
                        &temp_full_path,
                        &local_path
                    );
                    fs::rename(temp_full_path.clone(), local_path.clone())
                        .map_err(AppError::FSErrors)?;

                    log::info!(
                        "Success moved downloaded file from tmp: {} to local disk path: {}",
                        &temp_full_path,
                        &local_path
                    );
                };

                Result::<(), AppErrorUnit>::Ok(())
            }))
        } else {
            Err(AppError::LogicalError(format!(
                "Only cloud file can be downloaded, try download {:?}",
                file_to_download
            )))
        }
    }
}
