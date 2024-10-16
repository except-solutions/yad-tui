use yad_tui::{
    config::{Api, Config, DebugLevel, Main, MetaDb},
    meta_db::Meta,
    model::{File, Model, NodeType},
};

#[test]
fn test_wrong_indexes() {
    let mut m = setup();
    let too_small_index = -1;
    let too_big_index = (m.current_dir.len() + 1) as i32;
    assert_eq!(
        m.set_active_file(too_small_index),
        Err(format!("Inavlid index {too_small_index}"))
    );
    assert_eq!(
        m.set_active_file(too_big_index),
        Err(format!("Inavlid index {too_big_index}"))
    );
}

fn setup() -> Model {
    let previous = vec![File {
        name: String::from("abc"),
        active: true,
        file_type: NodeType::File,
    }];
    let next = vec![File {
        name: String::from("abc"),
        active: true,
        file_type: NodeType::File,
    }];
    let current_dir = vec![
        File {
            name: String::from("abc"),
            active: true,
            file_type: NodeType::Dir,
        },
        File {
            name: String::from("def"),
            active: false,
            file_type: NodeType::File,
        },
        File {
            name: String::from("xyz"),
            active: false,
            file_type: NodeType::File,
        },
        File {
            name: String::from("abc"),
            active: false,
            file_type: NodeType::Dir,
        },
    ];

    Model {
        active_file_row_index: 0,
        popup: Default::default(),
        previous_dir: previous,
        current_dir,
        sub_dir: next,
        config_path: Default::default(),
        config: Config {
            main: Main {
                sync_dir_path: "".to_string(),
                log_level: DebugLevel::Debug,
                cahce_dir_path: "".to_string(),
            },
            api: Api {
                api_url: "".to_string(),
                oauth_url: "".to_string(),
                client_id: "".to_string(),
                client_secret: "".to_string(),
            },
            meta_db: MetaDb {
                path: "".to_string(),
            },
        },
        meta: Meta { api_token: None },
        disk_client: yad_tui::disk_client::DiskClient {
            api_url: "".to_string(),
            oauth_url: "".to_string(),
            client_id: "".to_string(),
            client_secret: "".to_string(),
            token: None,
        },
    }
}
