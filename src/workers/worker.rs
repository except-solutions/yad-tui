use std::time::Duration;
use crate::{disk_client::DiskClientT, models::model::Model};
// Worker represent any background periodic task runnable by Scheduler
pub trait Worker: Clone {
    // Value that should be asynced passed into channel Sender, ex:
    // type SenderValueType = (DirItem, DirItems);
    //
    type SenderValueType;
    // Min interval between runs in seconds:
    // 
    //fn interval(&self) -> Duration {
    //  Duration::new(60, 0)
    //}
    fn interval(&self) -> Duration;
    // Shoud return value to calculate time delta between runs
    fn previous_run_time(&self) -> u64;
    // Run worker, should be non blocking
    fn run<T>(&self, model: &mut Model<T>) -> Self where T: DiskClientT;
    fn handle<T>(self, model: &mut Model<T>) -> Self where T: DiskClientT;
    fn blocked(&self) -> bool;
}

