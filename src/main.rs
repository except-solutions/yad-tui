use std::io::{self, stdout};

use ratatui::{
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
};

use yad_tui::config::{get_real_config_path, get_toml_config};
use yad_tui::events::handle_events;
use yad_tui::fs;
use yad_tui::meta_db::init_db;
use yad_tui::model::*;
use yad_tui::ui::ui;
use yad_tui::update::update;
use yad_tui::{cli::parse_args, disk_client::DiskClient};

use log::info;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

fn init() -> Model {
    let args = parse_args();

    let config = get_toml_config(&args.conf);
    let (meta_db, meta) = init_db(&config);
    let current_dirs = fs::get_init_fs_tree(&config.main.sync_dir_path);
    let disk_client = DiskClient::from_app_conf(&config);

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

    let log_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} [{l}] - {m}{n}")))
        .build(format!("{}log/app.log", config.main.cahce_dir_path))
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

    Model {
        active_file_row_index: 0,
        previous_dir: previous,
        current_dir: current_dirs,
        sub_dir: next,
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
        meta,
        disk_client,
    }
}

fn main() -> io::Result<()> {
    let mut model = init();
    info!("Start application");
    info!("Initialize application model");
    log::debug!("Initializated model: {:?}", model);
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
