pub mod cli;
pub mod components;
pub mod config;
pub mod disk_client;
pub mod events;
pub mod fs;
pub mod meta_db;
pub mod models;
pub mod ui;
pub mod update;
pub mod updaters;
pub mod utils;

#[macro_use]
extern crate rust_i18n;
i18n!("locales");
