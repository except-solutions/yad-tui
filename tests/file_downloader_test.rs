use std::io::BufReader;
use std::sync::{mpsc, Arc};

use std::{fs, panic};
use yad_tui::{
    config::{Api, Config, DebugLevel, Main, MetaDb},
    disk_client::DiskClientT,
    models::file::{CloudFile, File, NodeType, State},
    utils::{dir_reader::DirReader, file_downloader::FileDownloader},
};

#[derive(Clone)]
struct DiskClientMock {
    fake_file_path: String,
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
        todo!()
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
        let buf_reader = BufReader::new(file);
        Ok(Box::new(buf_reader))
    }

    fn update_token(&self, new_token: String) -> Self {
        todo!()
    }
}

fn setup() -> Arc<Config> {
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
            sync_dir_path: String::from("./tmp/sync_dir/"),
            log_level: DebugLevel::Debug,
            cache_dir_path: "./tmp/cache".to_string(),
        },
    });
    let _ = fs::create_dir_all("./tmp");
    fs::create_dir_all(config.main.sync_dir_path.clone()).unwrap();
    config
}

#[test]
fn test_single_file_download() {
    let config = setup();

    let result = panic::catch_unwind(|| {
        let disk_client = Arc::new(DiskClientMock {
            fake_file_path: "tests/fixtures/fake_file.ext".to_string(),
        });
        let dir_reader = Arc::new(DirReader {
            sync_dir_path: config.main.sync_dir_path.clone(),
            disk_client: Arc::clone(&disk_client),
        });

        let (download_file_sender, _) = mpsc::channel::<usize>();

        let file_downloader = FileDownloader {
            config: config.clone(),
            dir_reader,
            disk_client,
        };

        let file_to_download = File {
            name: "test_file.ext".to_string(),
            file_type: NodeType::File,
            state: State::Cloud,
            cloud: Some(CloudFile {
                path: "test_f_name.ext".to_string(),
            }),
            local: None,
        };

        let _ = file_downloader
            .download(download_file_sender, file_to_download.clone())
            .unwrap()
            .join()
            .unwrap()
            .unwrap();

        let test_file_stat = fs::metadata("tests/fixtures/fake_file.ext".to_string()).unwrap();
        let result_file_stat = fs::metadata(format!(
            "{}{}",
            config.main.sync_dir_path,
            file_to_download.cloud.unwrap().path
        ))
        .unwrap();

        assert_eq!(test_file_stat.len(), result_file_stat.len());
        assert!(result_file_stat.is_file());
    });

    result.unwrap();
}

#[test]
fn test_dir_download() {
    let config = setup();

    let result = panic::catch_unwind(|| {
        let disk_client = Arc::new(DiskClientMock {
            fake_file_path: "tests/fixtures/fake_dir.zip".to_string(),
        });
        let dir_reader = Arc::new(DirReader {
            sync_dir_path: config.main.sync_dir_path.clone(),
            disk_client: Arc::clone(&disk_client),
        });

        let (download_file_sender, _) = mpsc::channel::<usize>();

        let file_downloader = FileDownloader {
            config: config.clone(),
            dir_reader,
            disk_client,
        };

        let file_to_download = File {
            name: "fake_dir.zip".to_string(),
            file_type: NodeType::Dir,
            state: State::Cloud,
            cloud: Some(CloudFile {
                path: "test_dir_name".to_string(),
            }),
            local: None,
        };

        let _ = file_downloader
            .download(download_file_sender, file_to_download.clone())
            .unwrap()
            .join()
            .unwrap()
            .unwrap();

        let dir_path = format!(
            "{}{}",
            config.main.sync_dir_path,
            file_to_download.cloud.unwrap().path
        );
        let result_dir_stat = fs::metadata(dir_path.clone()).unwrap();
        let fake_file0 = fs::metadata(format!(
            "{}/{}",
            dir_path.clone(),
            "fake_file.ext".to_string()
        ))
        .unwrap();
        let fake_file1 = fs::metadata(format!(
            "{}/{}",
            dir_path.clone(),
            "inner_folder/inner_file.ext"
        ))
        .unwrap();

        assert!(result_dir_stat.is_dir());
        assert!(fake_file0.is_file());
        assert!(fake_file1.is_file());
    });

    fs::remove_dir_all("./tmp/sync_dir/test_dir_name").unwrap();
    result.unwrap();
}
