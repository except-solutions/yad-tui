use std::sync::mpsc::{Receiver, Sender};

use crate::{components::main_screen::next_dir::NextDir, error::AppError, models::model::Model};

pub struct Channels {
    pub read_next_dir_ch: ReadNextDirChannel,
    pub download_file_channel: DownloadFileChannel,
}

#[derive(Debug)]
pub struct ReadNextDirChannel {
    pub sender: Sender<Result<Option<NextDir>, AppError>>,
    pub receiver: Receiver<Result<Option<NextDir>, AppError>>,
}

pub trait Channel {
    fn handle(&self, model: &mut Model);
}

impl Channel for ReadNextDirChannel {
    fn handle(&self, model: &mut Model) {
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
    fn handle(&self, _: &mut Model) {
        if let Ok(bytes) = &self.receiver.try_recv() {
            // TODO: Add downloaded bytes in model
            // bytes.clone();
        };
    }
}
