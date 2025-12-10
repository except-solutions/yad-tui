    use yad_tui::workers::scheduler::Scheduler;

    use yad_tui::workers::worker::Worker;

    use yad_tui::disk_client::DirItems;

    use yad_tui::disk_client::DirItem;

    use yad_tui::workers::update_current_dir::UpdateCurrentDir;

    use ratatui::{
        crossterm::{
            terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
            ExecutableCommand,
        },
        prelude::*,
    };

    use std::sync::mpsc;

    use std::{
        fs,
        io::{self, stdout},
        sync::Arc,
    };

    use yad_tui::{channels::{RefreshDirChannel, DownloadFileChannel}, components::main_screen::current_dir::CurrentDir, ui::ui, workers::update_current_dir};

    use yad_tui::{
        channels::{Channel, Channels, ReadNextDirChannel},
        components::main_screen::next_dir::NextDir,
        error::AppError,
        meta_db::init_db,
    };

    use yad_tui::{cli::parse_args, disk_client::DiskClient};

    use yad_tui::{
        components::main_screen::top_bar::TopBar,
        config::{get_real_config_path, get_toml_config},
        models::disk_meta::DiskMeta,
    };

    use yad_tui::{disk_client::DiskClientT, update::update, utils::file_downloader::FileDownloader};

    use yad_tui::{events::handle_events, utils::dir_reader::DirReader};

    use log::{debug, info};

    use log4rs::append::file::FileAppender;

    use log4rs::config::{Appender, Config, Root};

    use log4rs::encode::pattern::PatternEncoder;

    use yad_tui::fs::FS;

    use yad_tui::models::model::{Model, Popup};

    #[macro_use]
    pub(crate) extern crate rust_i18n;

    i18n!("locales");

    pub(crate) fn main() -> io::Result<()> {
        let parse_args = parse_args();
        let args = parse_args;
        let config = get_toml_config(&args.conf);

        rust_i18n::set_locale(config.main.lang.as_str());

        let (meta_db, meta) = init_db(&config);
        let disk_client: Arc<DiskClient> = Arc::new(DiskClient::from_app_conf(&config, &meta));
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

        fs::create_dir_all(format!("{}/tmp", config.main.cache_dir_path)).unwrap();

        let dir_reader = Arc::new(DirReader {
            sync_dir_path: config.main.sync_dir_path.clone(),
            disk_client: disk_client.clone(),
        });

        let top_bar = meta.api_token.clone().map(|_| {
            let disk_meta = DiskMeta::from(disk_client.disk_meta().unwrap());
            TopBar { disk_meta }
        });

        let (fs_sender, fs_receiver) = mpsc::channel::<Result<Option<NextDir>, AppError>>();

        let ch = ReadNextDirChannel {
            sender: fs_sender,
            receiver: fs_receiver,
        };

        let (download_file_sender, download_file_receiver) = mpsc::channel::<usize>();

        let download_file_channel = DownloadFileChannel {
            sender: download_file_sender,
            receiver: download_file_receiver,
        };

        let (chg_open_dir_sender, chg_open_dir_receiver) =  mpsc::channel::<(DirItem, DirItems)>();


        let refresh_dir_ch = RefreshDirChannel {
            sender: chg_open_dir_sender,
            receiver: chg_open_dir_receiver

        };

        let channels = Channels {
            read_next_dir_ch: ch,
            download_file_channel,
        //    refresh_dir_ch
        };

        let config_pointer = Arc::new(config.clone());

        let fd = Arc::new(FileDownloader {
            config: Arc::clone(&config_pointer),
            dir_reader: Arc::clone(&dir_reader),
            disk_client: Arc::clone(&disk_client),
        });

        let fs = FS::create(Arc::clone(&config_pointer), dir_reader, fd).unwrap();
        let (mut model, channels) = (
            Model {
                is_auth: meta.api_token.is_some(),
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
            },
            channels,
        );

        let update_current_dir_worker = UpdateCurrentDir::new(&refresh_dir_ch);

        let mut scheduler = Scheduler { workers:  vec![update_current_dir_worker] };

        info!("Initialize application model");
        debug!("Initializated model: {:?}", model);
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        let mut current_message = handle_events(&model)?;
    //    let mut update_current_dir = UpdateCurrentDir::new();

        while current_message.is_some() {
            terminal.draw(|f| ui(&mut model, f))?;
            current_message = match handle_events(&model)? {
                Some(m) => update(&mut model, m, &channels),
                None => None,
            };

            let _ = &channels.read_next_dir_ch.handle(&mut model);
            // TODO: Impl handling download progress
            channels.download_file_channel.handle(&mut model);
     //       update_current_dir = update_current_dir.update(&model, channels.refresh_dir_ch.sender.clone());
            channels.read_next_dir_ch.handle(&mut model);


    //        update_current_dir_worker.run(&mut model);
//            refresh_dir_ch.handle(&mut model);
            scheduler.run(&mut model);

//            scheduler = Scheduler { workers: &workers };


    //        update_current_dir_worker.run(&mut model);

             refresh_dir_ch.handle(&mut model);

        }

        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }
