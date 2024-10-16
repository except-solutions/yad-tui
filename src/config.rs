use crate::model::Model;
use log::{self, LevelFilter};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Api {
    pub api_url: String,
    pub oauth_url: String,
    pub client_id: String,
    pub client_secret: String,
}

impl Api {
    pub fn auth_link(&self) -> String {
        format!(
            "{0}/authorize?response_type=code&client_id={1}",
            self.oauth_url, self.client_id
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum DebugLevel {
    Info,
    Debug,
}

impl DebugLevel {
    pub fn to_level_filter(&self) -> LevelFilter {
        match &self {
            DebugLevel::Info  => LevelFilter::Info,
            DebugLevel::Debug => LevelFilter::Debug
        }
    }
}


impl FromStr for DebugLevel {
    type Err = String;

    fn from_str(input: &str) -> Result<DebugLevel, Self::Err> {
        match input {
            "info" => Ok(DebugLevel::Info),
            "debug" => Ok(DebugLevel::Debug),
            invalid => Err(format!("Invalid debug level {}", invalid)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Main {
    pub sync_dir_path: String,
    pub log_level: DebugLevel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaDb {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api: Api,
    pub meta_db: MetaDb,
    pub main: Main,
}

pub fn get_toml_config(path: &String) -> Config {
    let real_path = get_real_config_path(path);
    log::info!(
        "Try read log file from path: {:?}",
        real_path.clone().into_os_string()
    );
    let toml_data = std::fs::read_to_string(real_path.clone()).unwrap_or_else(|_| {
        panic!(
            "{}",
            format!(
                "Cannot read config file '{:?}'. Please run yad --init-conf",
                real_path.clone()
            )
            .to_owned()
        )
    });
    toml::from_str(&toml_data).unwrap()
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

