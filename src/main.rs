use crate::Message::*;
use ratatui::crossterm::event::KeyEventKind;
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    style::palette::tailwind,
    widgets::*,
};
use std::io::{self, stdout};
use std::time::Duration;

#[derive(Debug, Default, Clone)]
struct File {
    name: String,
    active: bool,
}

#[derive(Debug, Default)]
struct Model {
    previous_dir: Vec<File>,
    current_dir: Vec<File>,
    sub_dir: Vec<File>,
    active_file_row_index: i32,
}

#[derive(PartialEq)]
enum Message {
    Exit,
    Continue,
    MoveDown,
    MoveUp,
}

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

fn to_list_item(file: &File) -> ListItem {
    if file.active {
        ListItem::new(file.name.clone()).bg(tailwind::RED.c950)
    } else {
        ListItem::new(file.name.clone())
    }
}

fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        MoveDown => {
            let current_dir_size = model.current_dir.len();
            let new_active_file_row_index_guess = model.active_file_row_index + 1;
            model.active_file_row_index =
                if current_dir_size <= new_active_file_row_index_guess as usize {
                    model.active_file_row_index
                } else {
                    new_active_file_row_index_guess
                };
            model.current_dir = model
                .current_dir
                .iter()
                .enumerate()
                .map(|(i, file)| File {
                    active: i == model.active_file_row_index as usize,
                    ..file.clone()
                })
                .collect();
            Some(Continue)
        }
        MoveUp => Some(Continue),
        Exit => None,
        Continue => Some(Continue),
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

fn handle_events() -> io::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                use KeyCode::*;
                use Message::*;
                return match key.code {
                    Char('q') | Esc => Ok(Some(Exit)),
                    Char('j') | Down => Ok(Some(MoveDown)),
                    Char('k') | Up => Ok(Some(MoveUp)),
                    _ => Ok(Some(Continue)),
                };
            }
        }
    }
    Ok(Some(Message::Continue))
}

fn ui(model: &mut Model, frame: &mut Frame) {
    let vertical_layouts = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(frame.size())
        .to_vec();
    let previous_dir_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let current_dir_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let next_dir_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let previous_dir = List::new(["root dir"]).block(previous_dir_block);
    let current_dir =
         List::new(model.current_dir.iter().map(|f| to_list_item(f))).block(current_dir_block);
    let next_dir = List::new(["z", "z"]).block(next_dir_block);
    frame.render_widget(previous_dir, vertical_layouts[0]);

    frame.render_widget(current_dir, vertical_layouts[1]);
    frame.render_widget(next_dir, vertical_layouts[2]);
}
