use crate::workers::worker::Worker;
use std::time::Duration;
use crate::error::AppError;
use crate::disk_client::DirItems;
use std::sync::mpsc::Sender;

use crate::disk_client::DirItem;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use std::thread;

use crate::{disk_client::DiskClientT, models::model::Model};

#[derive(Clone, Debug)]
pub struct UpdateCurrentDir {
    previous_update_time: u64,
}

impl Worker for UpdateCurrentDir {

    type SenderValueType = (DirItem, DirItems);

    fn run<T>(model: &Model<T>, sender: Sender<Self::SenderValueType>) 
        where 
            T: DiskClientT,
    {
        let disk_client = model.clone().disk_client;
        let current_dir_path = model.fs.current_dir.path.clone().into_os_string();
    
        thread::spawn(move || {
            let dir = disk_client.item(
                    current_dir_path.to_str().unwrap(),             
                    None,
                    None
                ).map_err(AppError::DiskErrors)?;

                // TODO map to app error
                sender.send(dir.as_tuple()).unwrap();

                Ok::<(), AppError>(())
            });

    }

    fn interval() -> Duration {
        Duration::new(60, 0)
    }
}

impl UpdateCurrentDir {

    pub fn new() -> Self {
        UpdateCurrentDir { 
            previous_update_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        } 
    }

    pub fn update<T: DiskClientT>(self, model: &Model<T>, sender: Sender<(DirItem, DirItems)>) -> UpdateCurrentDir {
        // calc current timeout
        // let mut current_dir = model.fs.current_dir.clone();
        //
        
            let max_delta = 10;
    
            let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let delta = current_time - self.previous_update_time;
    
            if delta > max_delta {
                let disk_client = model.clone().disk_client;
                let current_dir_path = model.fs.current_dir.path.clone().into_os_string();
    
                thread::spawn(move || {
                    let dir = disk_client.item(
                        current_dir_path.to_str().unwrap(),             
                        None,
                        None
                    ).map_err(AppError::DiskErrors)?;

                    // TODO map to app error
                    sender.send(dir.as_tuple()).unwrap();

                    Ok::<(), AppError>(())
                });

                UpdateCurrentDir { previous_update_time: current_time }
            } else {
                self.clone()
            }
    }
}


