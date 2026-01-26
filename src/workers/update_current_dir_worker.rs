pub use crate::channels::RefreshDirChannel;
use crate::disk_client::DirItems;
use crate::error::AppError;
pub(crate) use crate::error::AppErrorUnitOrT;
use crate::models::file::File;
use crate::workers::worker::Worker;
use std::path::PathBuf;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::disk_client::DirItem;
use std::thread;

use crate::{disk_client::DiskClientT, models::model::Model};

#[derive(Clone, Debug)]
pub struct UpdateCurrentDirWorker<'a> {
    pub previous_update_time: u64,
    pub blocked: bool,
    pub channel: &'a RefreshDirChannel,
}

impl<'a> Worker for UpdateCurrentDirWorker<'a> {
    type SenderValueType = (DirItem, DirItems);
    type RunResultType =
        thread::JoinHandle<Result<(), AppErrorUnitOrT<(PathBuf, (File, Vec<File>))>>>;

    fn run<T>(&self, model: &mut Model<T>) -> (Self, Self::RunResultType)
    where
        T: DiskClientT,
    {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let current_dir = model.fs.current_dir.clone();
        let current_dir_path = current_dir.path.clone();
        let sender = self.channel.sender.clone();
        let reader = model.fs.dir_reader.clone();

        let t = thread::spawn(move || {
            let dir = reader
                .read_dir(current_dir_path.to_str().unwrap().to_string())
                .map_err(AppErrorUnitOrT::AppErrorUnit)?;

            sender
                .send((current_dir_path, dir))
                .map_err(|e| AppErrorUnitOrT::AppErrorT(AppError::SendError(e)))
        });

        let new_worker = UpdateCurrentDirWorker {
            previous_update_time: current_time,
            blocked: true,
            channel: self.channel,
        };

        (new_worker, t)
    }

    fn interval(&self) -> Duration {
        Duration::new(30, 0)
    }

    fn previous_run_time(&self) -> u64 {
        self.previous_update_time
    }

    fn handle<T>(self, model: &mut Model<T>) -> Self
    where
        T: DiskClientT,
    {
        if let Ok((path, (f, i))) = self.channel.receiver.try_recv() {
            if path == model.fs.current_dir.path {
                model.fs.current_dir = model.fs.current_dir.update(f, i);
            };

            UpdateCurrentDirWorker {
                previous_update_time: self.previous_update_time,
                blocked: false,
                channel: self.channel,
            }
        } else {
            self
        }
    }

    fn blocked(&self) -> bool {
        self.blocked
    }
}

impl<'a> UpdateCurrentDirWorker<'a> {
    pub fn new(channel: &'a RefreshDirChannel) -> Self {
        UpdateCurrentDirWorker {
            previous_update_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            blocked: false,
            channel,
        }
    }
}
