use std::sync::mpsc::Sender;
use std::time::Duration;
use crate::{disk_client::DiskClientT, models::model::Model};

pub trait Worker {

    type SenderValueType;

    fn interval() -> Duration;
    fn run<T>(model: &Model<T>, sender: Sender<Self::SenderValueType>) where T: DiskClientT;
}
