use crate::components::main_screen::current_dir::CurrentDir;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use crate::channels::RefreshDirChannel;
use crate::workers::worker::Worker;
use std::time::Duration;
use crate::error::AppError;
use crate::disk_client::DirItems;

use crate::disk_client::DirItem;
use std::thread;

use crate::{disk_client::DiskClientT, models::model::Model};

#[derive(Clone, Debug)]
pub struct UpdateCurrentDir<'a> {
    pub previous_update_time: u64,
    pub blocked: bool,
    pub channel: &'a RefreshDirChannel
}

impl <'a>Worker for UpdateCurrentDir<'a> {

    type SenderValueType = (DirItem, DirItems);

    fn run<T>(&self, model: &mut Model<T>) -> Self
        where 
            T: DiskClientT,
    {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let disk_client = model.clone().disk_client;
        let current_dir_path = model.fs.current_dir.path.clone().into_os_string();
        let sender = self.channel.sender.clone();
    
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
        UpdateCurrentDir {
            previous_update_time: current_time,
            blocked: true,
            channel: self.channel
        }
    }

    fn interval(&self) -> Duration {
        Duration::new(5, 0)
    }
    
    fn previous_run_time(&self) -> u64 {
        self.previous_update_time
    }

    fn handle<T>(self, model: &mut Model<T>) -> Self where T: DiskClientT {
//        println!("Handler ");
        if let Ok(d) = self.channel.receiver.try_recv() {

            println!("dir: {:?}", d.0);

            //model.fs.current_dir = model.fs.current_dir.clone()
            //
            // let new_dir = CurrentDir::new(model.fs.current_dir.path.clone(), d.0, d.1);


            UpdateCurrentDir {
                previous_update_time: self.previous_update_time,
                blocked: false,
                channel: self.channel
            }
        } else {
            self
        }
    }

    fn blocked(&self) -> bool {
        self.blocked
    }
}

impl <'a>UpdateCurrentDir<'a> {

    pub fn new(channel: &'a RefreshDirChannel) -> Self {
        UpdateCurrentDir {
            previous_update_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            blocked: false,
            channel
        }
    }
}

