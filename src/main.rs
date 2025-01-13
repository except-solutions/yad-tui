use ratatui::{
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
};
use std::io::{self, stdout};

use yad_tui::meta_db::init_db;
use yad_tui::ui::ui;
use yad_tui::update::update;
use yad_tui::{cli::parse_args, disk_client::DiskClient};
use yad_tui::{
    components::main_screen::top_bar::TopBar,
    config::{get_real_config_path, get_toml_config},
    models::disk_meta::DiskMeta,
};
use yad_tui::{events::handle_events, utils::dir_reader::DirReader};

use log::{debug, info};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use yad_tui::fs::{ReaderHOF, FS};
use yad_tui::models::model::{Model, Popup};

#[macro_use]
extern crate rust_i18n;

i18n!("locales");

fn init() -> Model {
    let args = parse_args();
    let config = get_toml_config(&args.conf);

    rust_i18n::set_locale(config.main.lang.as_str());

    let (meta_db, meta) = init_db(&config);
    let disk_client = DiskClient::from_app_conf(&config, &meta);

    let log_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} [{l}] - {m}{n}")))
        .build(format!("{}log/app.log", config.main.cache_dir_path))
        .unwrap();

    let log_config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(log_file)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(config.main.log_level.to_level_filter()),
        )
        .unwrap();
    log4rs::init_config(log_config).unwrap();
    // let dr = disk_client.item("/", None, Some(1000)).unwrap();

    let dir_reader = DirReader {
        sync_dir_path: config.main.sync_dir_path.clone(),
        disk_client: disk_client.clone(),
    };

    let fs = FS::create(dir_reader);

    let top_bar = meta.api_token.clone().map(|_| {
        let disk_meta = DiskMeta::from(disk_client.disk_meta().unwrap());
        TopBar { disk_meta }
    });

    Model {
        top_bar,
        fs,
        config,
        popup: if meta.api_token.is_some() {
            None
        } else {
            Some(Popup::LoginForm {
                code_input: "".to_string(),
                error_message: None,
            })
        },
        config_path: get_real_config_path(&args.conf),
        meta_db,
        disk_client,
    }
}

fn main() -> io::Result<()> {
    let mut model = init();
    info!("Start application");
    info!("Initialize application model");
    debug!("Initializated model: {:?}", model);
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut current_message = handle_events(&model)?;

    while current_message.is_some() {
        terminal.draw(|f| ui(&mut model, f))?;
        current_message = match handle_events(&model)? {
            Some(m) => update(&mut model, m),
            None => None,
        };
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
