use crate::model::Model;
use crate::update::InputAction::*;
use crate::update::Message;
use crate::update::Message::*;
use ratatui::crossterm::event;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::{Event, KeyEventKind};
use std::io;
use std::time::Duration;

pub fn handle_events(model: &Model) -> io::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                use KeyCode::*;

                let message = if model.popup.is_some() {
                    match key.code {
                        Char('q') | Esc => Some(ClosePopup),
                        Char(word) => Some(InputModeAction(InputChar(word))),
                        Backspace => Some(InputModeAction(DeleteChar)),
                        Enter => Some(InputModeAction(Send)),
                        _ => Some(Continue),
                    }
                } else {
                    match key.code {
                        Char('q') | Esc => Some(Exit),
                        Char('j') | Down => Some(MoveDown),
                        Char('k') | Up => Some(MoveUp),
                        Char('c') => Some(ShowConfig),
                        _ => Some(Continue),
                    }
                };

                return Ok(message);
            }
        }
    }
    Ok(Some(Message::Continue))
}
