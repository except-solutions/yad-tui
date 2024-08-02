use std::io::{self, stdout};

use ratatui::{
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
};

use yad_tui::events::handle_events;
use yad_tui::model::*;
use yad_tui::ui::ui;
use yad_tui::update::{update, Message};

use crate::Message::*;

fn init() -> Model {
    let previous = vec![File {
        name: String::from("abc"),
        active: true,
    }];
    let next = vec![File {
        name: String::from("abc"),
        active: true,
    }];
    let current_dir = vec![
        File {
            name: String::from("abc"),
            active: true,
        },
        File {
            name: String::from("def"),
            active: false,
        },
        File {
            name: String::from("xyz"),
            active: false,
        },
        File {
            name: String::from("abc"),
            active: false,
        },
    ];

    Model {
        active_file_row_index: 0,
        previous_dir: previous,
        current_dir,
        sub_dir: next,
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut model = init();
    let mut current_message = handle_events()?;
    while current_message.is_some() {
        terminal.draw(|f| ui(&mut model, f))?;

        current_message = match handle_events()? {
            Some(m) => update(&mut model, m),
            None => None,
        };
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
