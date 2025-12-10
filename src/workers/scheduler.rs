pub(crate) use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use crate::disk_client::DiskClientT;
use crate::models::model::Model;
use crate::workers::worker::Worker;

pub struct Scheduler<T: Worker> {

    pub workers: Vec<T>
}

impl <T: Worker> Scheduler<T> {
    
    pub fn run<D>(&mut self, model: &mut Model<D>) where D: DiskClientT,
    {
        self.workers = self
            .workers
            .iter()
            .map(|w| {
                let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                let interval = w.interval().as_secs();

                let delta = current_time - w.previous_run_time();

                let runned_or_skipped_worker = if !w.blocked() && delta > interval {
                   w.run(model)
                } else {
                    w.clone()
                };

                runned_or_skipped_worker.handle(model)
            })
            .collect();
    }
}

