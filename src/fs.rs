use crate::components::main_screen::next_dir::NextDir;
use crate::components::main_screen::{current_dir::CurrentDir, previous_dir::PreviousDir};
use crate::config::Config;
use crate::disk_client::DiskClientT;
use crate::error::AppError;
use crate::models::file::{File, NodeType};
use crate::utils::common::path_buf_to_string;
use crate::utils::dir_reader::DirReader;
use crate::utils::file_downloader::FileDownloader;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;

type NextDirSetResult = Result<Option<NextDir>, AppError>;
type FSSender = Sender<NextDirSetResult>;
#[derive(Debug, Clone)]
pub struct FS<T: DiskClientT> {
    pub previous_dir: Option<PreviousDir>,
    pub current_dir: CurrentDir,
    pub next_dir: Option<NextDir>,
    pub dir_reader: Arc<DirReader<T>>,
    pub file_downloader: Arc<FileDownloader<T>>,
    pub config: Arc<Config>,
}

impl<T> FS<T>
where
    T: DiskClientT,
{
    pub fn create(
        config: Arc<Config>,
        dir_reader: Arc<DirReader<T>>,
        file_downloader: Arc<FileDownloader<T>>,
    ) -> Result<Self, AppError> {
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

        Ok(Self::new(
            config,
            previous_dir,
            current_dir,
            next_dir,
            dir_reader,
            file_downloader,
        ))
    }
}

impl<T> FS<T>
where
    T: DiskClientT,
{
    pub fn new(
        config: Arc<Config>,
        previous_dir: Option<PreviousDir>,
        current_dir: CurrentDir,
        next_dir: Option<NextDir>,
        dir_reader: Arc<DirReader<T>>,
        file_downloader: Arc<FileDownloader<T>>,
    ) -> Self {
        Self {
            config,
            next_dir,
            current_dir,
            previous_dir,
            dir_reader,
            file_downloader,
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

        self.file_downloader.download(sender, selected.clone())?;
        Ok(())
    }
}
