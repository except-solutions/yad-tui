use std::{fs, path::PathBuf, sync::Arc};

use jammdb::DB;
use yad_tui::{
    components::main_screen::current_dir::CurrentDir,
    config::{Api, Config, DebugLevel, Main, MetaDb},
    disk_client::{DiskClientT, ItemResponse},
    fs::FS,
    models::{
        file::{File, NodeType, State},
        model::Model,
    },
    utils::{dir_reader::DirReader, file_downloader::FileDownloader},
};

#[derive(Clone)]
pub struct DiskClientMock {
    pub fake_file_path: String,
    pub fake_item_response: ItemResponse,
}

impl DiskClientT for DiskClientMock {
    fn auth(&self, _code: String) -> Result<yad_tui::disk_client::SuccessAuth, String> {
        todo!()
    }

    fn disk_meta(
        &self,
    ) -> Result<yad_tui::disk_client::DiskMetaResponse, yad_tui::disk_client::DiskError> {
        todo!()
    }

    fn item(
        &self,
        _path: &str,
        _offset: Option<u32>,
        _limit: Option<u32>,
    ) -> Result<yad_tui::disk_client::ItemResponse, yad_tui::disk_client::DiskError> {
        Ok(self.fake_item_response.clone())
    }

    fn prepare_request(
        &self,
        _request_f: impl Fn() -> ureq::Request,
    ) -> Result<ureq::Request, yad_tui::disk_client::DiskError> {
        todo!()
    }

    fn set_api_token(
        &self,
        _request: ureq::Request,
    ) -> Result<ureq::Request, yad_tui::disk_client::DiskError> {
        todo!()
    }

    fn file_reader(
        &self,
        _cloud_path: String,
    ) -> Result<Box<dyn std::io::Read + Send + Sync>, yad_tui::disk_client::DiskError> {
        let file = fs::File::open(self.fake_file_path.clone()).unwrap();
        let reader = std::io::BufReader::new(file);
        Ok(Box::new(reader))
    }

    fn update_token(&self, _new_token: String) -> Self {
        todo!()
    }
}

fn build_config() -> Config {
    let config = Config {
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
            sync_dir_path: String::from("./tmp/sync_dir/"),
            log_level: DebugLevel::Debug,
            cache_dir_path: "./tmp/cache".to_string(),
        },
    };
    let _ = fs::create_dir_all("./tmp");
    fs::create_dir_all(config.main.sync_dir_path.clone()).unwrap();
    config
}

#[track_caller]
pub fn create_test_model(
    fake_disk_item_response: ItemResponse,
) -> (Model<DiskClientMock>, PathBuf) {
    let config = build_config();
    let config_arc = Arc::new(config.clone());

    let disk_client = Arc::new(DiskClientMock {
        fake_file_path: "tests/fixtures/fake_file.ext".to_string(),
        fake_item_response: fake_disk_item_response.clone(),
    });

    let dir_reader = Arc::new(DirReader {
        sync_dir_path: config.main.sync_dir_path.clone(),
        disk_client: Arc::clone(&disk_client),
    });

    let file_downloader = Arc::new(FileDownloader {
        config: Arc::clone(&config_arc),
        dir_reader: Arc::clone(&dir_reader),
        disk_client: Arc::clone(&disk_client),
    });

    let current_path = PathBuf::from("/current");
    let current_dir = CurrentDir::new(
        current_path.clone(),
        File {
            name: fake_disk_item_response.name.clone(),
            file_type: NodeType::Dir,
            state: State::Local,
            cloud: None,
            local: None,
        },
        vec![File {
            name: "old_file".to_string(),
            file_type: NodeType::File,
            state: State::Local,
            cloud: None,
            local: None,
        }],
    );

    let fs_state = FS::new(
        Arc::clone(&config_arc),
        None,
        current_dir,
        None,
        Arc::clone(&dir_reader),
        Arc::clone(&file_downloader),
    );

    let _ = fs::create_dir_all("./tmp");
    let meta_db = DB::open("./tmp/update_current_dir_worker_meta.db").unwrap();

    let model = Model {
        is_auth: false,
        top_bar: None,
        fs: fs_state,
        popup: None,
        config,
        config_path: PathBuf::from("./tmp/config.toml"),
        meta_db,
        disk_client,
    };

    (model, current_path)
}
