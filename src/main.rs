use std::io::{self, stdout};

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;

    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame) {
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
    let current_dir = List::new(["x", "y"]).block(current_dir_block);
    let next_dir = List::new(["z", "z"]).block(next_dir_block);
    frame.render_widget(previous_dir, vertical_layouts[0]);

    frame.render_widget(current_dir, vertical_layouts[1]);
    frame.render_widget(next_dir, vertical_layouts[2]);
}
