use crate::config::Config;
use jammdb::DB;
use std::{env::home_dir, fs, path::Path};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Meta {
    pub api_token: Option<String>,
}

pub fn init_db(config: &Config) -> (DB, Meta) {
    let home = home_dir();
    let path = home.unwrap().join(Path::new(&config.meta_db.path[..]));
    let app_cache_dir = path.parent().unwrap();

    if !app_cache_dir.exists() {
        fs::create_dir_all(app_cache_dir).unwrap();
    }
    let db = DB::open(path).unwrap();
    let tx = db.tx(true).unwrap();
    let meta_b = tx.get_or_create_bucket("meta").unwrap();

    let meta = match meta_b.get("meta") {
        Some(data) => serde_json::from_slice::<Meta>(data.kv().value()).unwrap(),
        None => Meta { api_token: None },
    };

    (db.clone(), meta)
}
