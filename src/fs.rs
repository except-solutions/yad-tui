use crate::components::main_screen::next_dir::NextDir;
use crate::components::main_screen::{current_dir::CurrentDir, previous_dir::PreviousDir};
use crate::error::AppError;
use crate::models::file::{CloudFile, File, NodeType};
use crate::utils::common::path_buf_to_string;
use crate::utils::dir_reader::DirReader;
use crate::utils::progress_file_reader::ProgressFileReader;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use log;

type NextDirSetResult = Result<Option<NextDir>, AppError>;
type FSSender = Sender<NextDirSetResult>;

#[derive(Debug, Clone)]
pub struct FS {
    cache_dir_path: String,
    pub previous_dir: Option<PreviousDir>,
    pub current_dir: CurrentDir,
    pub next_dir: Option<NextDir>,
    pub dir_reader: DirReader,
}

impl FS {
    pub fn create(cache_dir_path: String, dir_reader: DirReader) -> Result<Self, AppError> {
        let path = PathBuf::from("/");
        let (item, items) = dir_reader.read_dir(path_buf_to_string(path.clone())?)?;
        let current_dir = CurrentDir::new(path.clone(), item, items);
        let previous_dir = None;

        let next_dir = if let Some(File {
            file_type: NodeType::Dir,
            name,
            ..
        }) = current_dir.items.first()
        {
            let next_path = path.clone().join(name.clone());
            let (item, items) = dir_reader.read_dir(path_buf_to_string(next_path.clone())?)?;
            Some(NextDir::new(next_path, item, items))
        } else {
            None
        };

        Ok(Self::new(cache_dir_path,  previous_dir, current_dir, next_dir, dir_reader))
    }
}

impl FS {
    pub fn new(
        cache_dir_path: String,
        previous_dir: Option<PreviousDir>,
        current_dir: CurrentDir,
        next_dir: Option<NextDir>,
        dir_reader: DirReader,
    ) -> Self {
        Self {
            cache_dir_path,
            next_dir,
            current_dir,
            previous_dir,
            dir_reader,
        }
    }

    pub fn select_next_element_for_next_dir(&mut self, sender: FSSender) -> Result<(), AppError> {
        self.current_dir.state.select_next();
        self.next_dir = None;

        let c = self.clone();

        thread::spawn(move || {
            let _ = sender.send(c.set_next_dir_from_current());
        });

        Ok(())
    }

    pub fn select_previous_element_for_next_dir(
        &mut self,
        sender: FSSender,
    ) -> Result<(), AppError> {
        self.current_dir.state.select_previous();
        self.next_dir = None;

        let c = self.clone();

        thread::spawn(move || {
            let _ = sender.send(c.set_next_dir_from_current());
        });

        Ok(())
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
                .read_dir(path_buf_to_string(path.clone())?)?;
            self.current_dir = CurrentDir::new(path.clone(), item, items.clone());
            self.next_dir = None;

            if !items.is_empty() {
                self.set_next_dir_from_current()?;
            };
        }

        Ok(())
    }

    pub fn open_previous(&mut self) -> Result<(), AppError> {
        if let Some(previous_dir) = &self.previous_dir {
            let (current_item, current_dir_items) = self
                .dir_reader
                .read_dir(path_buf_to_string(previous_dir.path.clone())?)?;

            self.current_dir =
                CurrentDir::new(previous_dir.path.clone(), current_item, current_dir_items);

            if let Some(prev_path) = previous_dir.path.parent() {
                let prev_path_buf = prev_path.to_path_buf();
                let (item, items) = self
                    .dir_reader
                    .read_dir(path_buf_to_string(prev_path_buf.clone())?)?;
                self.previous_dir = Some(PreviousDir::new(prev_path_buf, item, items));
            } else {
                self.previous_dir = None;
            }
        }

        Ok(())
    }

    fn set_next_dir_from_current(&self) -> NextDirSetResult {
        let selected = self.current_dir.selected_file()?;
        let next_dir = if selected.file_type == NodeType::Dir {
            let mut next_path_buf = PathBuf::new();
            next_path_buf.push(&self.current_dir.path);
            next_path_buf.push(&selected.name);
            let (item, items) = self
                .dir_reader
                .read_dir(path_buf_to_string(next_path_buf.clone())?)?;
            Some(NextDir::new(next_path_buf, item, items))
        } else {
            None
        };
        Ok(next_dir)
    }

    pub fn download_selected(&self, sender: Sender<usize>) -> Result<(), AppError> {
        // log::info!("Try download selected file: {}", self.current_dir.selected_file()?.name);
        let selected = self.current_dir.selected_file()?;
        // let downloadFile = File {  local: CloudFile {}};


        if let Some(cloud_file) = selected.cloud {
            let unix_time = SystemTime::now().duration_since(UNIX_EPOCH);
            let file_path = &cloud_file.path[5..];
            let temp_dir_path = format!("{}/tmp/{}_{:?}", self.cache_dir_path, file_path, unix_time.unwrap().as_secs());
            let temp_file_name = match selected.file_type {
                NodeType::File => selected.name,
                NodeType::Dir  => format!("{}.zip", selected.name)
            };
            // TODO: Implement unpacking
            let temp_full_path = format!("{}/{}", temp_dir_path, temp_file_name);
            fs::create_dir_all(temp_dir_path).unwrap();
            let disk_client = self.dir_reader.disk_client.clone();

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
                    fs::File::create(temp_full_path).map_err(AppError::FSErrors)?;

                std::io::copy(&mut wrapper, &mut temp_file).map_err(AppError::FSErrors)
            });
        };

        Ok(())
    }
}

