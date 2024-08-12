use crate::model::*;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Stylize;
use ratatui::style::palette::tailwind;
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem};
use ratatui::Frame;

pub fn ui(model: &mut Model, frame: &mut Frame) {
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

fn to_list_item(file: &File) -> ListItem {
    if file.active {
        ListItem::new(file.name.clone()).bg(tailwind::RED.c950)
    } else {
        ListItem::new(file.name.clone())
    }
}
