use std::{path::PathBuf, sync::mpsc};
mod support;

use yad_tui::{
    channels::RefreshDirChannel,
    disk_client::{DirItem, DirItems, ItemResponse},
    models::file::File,
    workers::{update_current_dir_worker::UpdateCurrentDirWorker, worker::Worker},
};

use crate::support::test_setup;

#[test]
fn refresh_worker_updates_current_dir() {
    let fake_disk_item_response = ItemResponse {
        name: "current".to_string(),
        resource_id: "test_resource_id".to_string(),
        path: "test_path".to_string(),
        r#type: "dir".to_string(),
        created: "2016-04-20T12:54:28+00:00".to_string(),
        modified: "2016-04-20T12:54:28+00:00".to_string(),
        revision: 1461156869222238,
        _embedded: DirItems {
            limit: 3,
            offset: 0,
            total: 10,
            items: vec![DirItem {
                name: "test_inner_file".to_string(),
                resource_id: "test_resource_id".to_string(),
                path: "test_path".to_string(),
                r#type: "dir".to_string(),
                created: "2016-04-20T12:54:28+00:00".to_string(),
                modified: "2016-04-20T12:54:28+00:00".to_string(),
                revision: 1461156869222238,
            }],
        },
    };
    let (mut model, current_path) = test_setup::create_test_model(fake_disk_item_response.clone());

    let (sender, receiver) = mpsc::channel::<(PathBuf, (File, Vec<File>))>();
    let refresh_dir_channel = RefreshDirChannel { sender, receiver };

    let worker = UpdateCurrentDirWorker::new(&refresh_dir_channel);

    let (new_worker, update_result) = worker.run(&mut model);

    assert!(new_worker.blocked);

    update_result.join().unwrap().unwrap();

    let updated_worker = worker.handle(&mut model);

    assert!(!updated_worker.blocked);

    assert_eq!(model.fs.current_dir.path, current_path);
    assert_eq!(model.fs.current_dir.item.name, fake_disk_item_response.name);
    assert_eq!(model.fs.current_dir.items.len(), 1);
    assert_eq!(
        model.fs.current_dir.items[0].name,
        fake_disk_item_response._embedded.items[0].name
    );
}
