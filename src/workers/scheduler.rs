pub(crate) use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use crate::disk_client::DiskClientT;
use crate::models::model::Model;
use crate::workers::worker::Worker;

pub struct Scheduler<'a, T: Worker> {

    pub workers: &'a Vec<T>
}

impl <'a, T: Worker> Scheduler<'a, T> {
    
    pub fn run<D>(&self, model: &mut Model<D>) -> Vec<T> where D: DiskClientT,
    {
        let updated_workers: Vec<T> = self
            .workers
            .into_iter()
            .map(|w| {

                let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                let interval = w.interval().as_secs();

                let delta = current_time - w.previous_run_time();

                if delta > interval {
                   w.run(model)
                } else {
                    w.clone()
                }
            })
            .collect();
        
        updated_workers
        
    }

}

