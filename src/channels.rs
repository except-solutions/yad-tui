use crate::models::file::File;
use crate::utils::common::AppErrorUnit;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};

use crate::{
    components::main_screen::next_dir::NextDir, disk_client::DiskClientT, models::model::Model,
};

pub struct Channels {
    pub read_next_dir_ch: ReadNextDirChannel,
    pub download_file_channel: DownloadFileChannel,
    //pub refresh_dir_ch: RefreshDirChannel
}

#[derive(Debug)]
pub struct ReadNextDirChannel {
    pub sender: Sender<Result<Option<NextDir>, AppErrorUnit>>,
    pub receiver: Receiver<Result<Option<NextDir>, AppErrorUnit>>,
}

pub trait Channel {
    fn handle<T>(&self, model: &mut Model<T>)
    where
        T: DiskClientT;
}

impl Channel for ReadNextDirChannel {
    fn handle<T>(&self, model: &mut Model<T>)
    where
        T: DiskClientT,
    {
        let result = &self.receiver.try_recv();
        if let Ok(Ok(next_dir)) = result {
            let selected = model.fs.current_dir.selected_file();

            if let (Some(n_dir), Ok(selected)) = (&next_dir, selected) {
                if n_dir.item.name == selected.name {
                    model.fs.next_dir = next_dir.clone()
                };
            };
        };
    }
}

#[derive(Debug)]
pub struct DownloadFileChannel {
    pub sender: Sender<usize>,
    pub receiver: Receiver<usize>,
}

impl Channel for DownloadFileChannel {
    fn handle<T>(&self, _: &mut Model<T>)
    where
        T: DiskClientT,
    {
        if let Ok(_bytes) = &self.receiver.try_recv() {
            // TODO: Add downloaded bytes in model
            // bytes.clone();
        };
    }
}

#[derive(Debug)]
pub struct RefreshDirChannel {
    pub sender: Sender<(PathBuf, (File, Vec<File>))>,
    pub receiver: Receiver<(PathBuf, (File, Vec<File>))>,
}

impl Channel for RefreshDirChannel {
    fn handle<T>(&self, _model: &mut Model<T>)
    where
        T: DiskClientT,
    {
        unimplemented!("Should be implemented in worker only!");
    }
}
