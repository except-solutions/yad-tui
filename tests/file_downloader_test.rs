use std::sync::Arc;

use yad_tui::{
    config::{Api, Config, Main, MetaDb},
    disk_client::DiskClient,
};

#[test]
fn test_single_file_download() {
    let config = Arc::new(Config {
        api: Api {
            api_url: String::from("test_api_url"),
            oauth_url: String::from("test_oauth_url"),
            client_id: String::from("test_client_id"),
            client_secret: String::from("client_secret"),
        },
        meta_db: MetaDb {
            path: String::from("test"),
        },
        main: Main {
            lang: String::from("en"),
            sync_dir_path: todo!(),
            log_level: todo!(),
            cache_dir_path: todo!(),
        },
    });

    assert_eq!(true, true);
}
