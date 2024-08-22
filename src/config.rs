use crate::model::Model;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use toml;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub main: HashMap<String, String>,
}

pub fn get_config_toml(path: &String) -> Config {
    let real_path = get_real_config_path(&path);
    let toml_data = std::fs::read_to_string(real_path)
        .expect("Cannot read config file. Please run yad --init-conf");
    let config: Config = toml::from_str(&*toml_data).unwrap();
    config
}

pub fn get_real_config_path(path: &String) -> PathBuf {
    if path.starts_with("/home/") {
        PathBuf::from(path)
    } else {
        let home_dir = home::home_dir();
        home_dir.expect("Cannot find $HOME dir").join(path)
    }
}

pub fn get_text_config(model: &mut Model) -> String {
    toml::to_string(&model.config).unwrap()
}
