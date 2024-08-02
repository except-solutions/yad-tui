use std::io;
use std::time::Duration;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyEventKind};
use ratatui::crossterm::event::KeyCode;
use crate::update::Message;
use crate::update::Message::{Continue, Exit, MoveDown, MoveUp};

pub fn handle_events() -> io::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                use KeyCode::*;
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