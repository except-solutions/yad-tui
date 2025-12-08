use std::sync::mpsc::Sender;
use std::time::Duration;
use crate::{disk_client::DiskClientT, models::model::Model};

pub trait Worker {

    type SenderValueType;

    fn interval(&self) -> Duration;
    fn previous_run_time(&self) -> u64;
    fn run<T>(&self, model: &mut Model<T>) -> Self where T: DiskClientT;
    fn handle(self) -> Self;
}
