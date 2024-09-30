use crate::config::get_text_config;
use crate::model::*;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Stylize;
use crate::{
    model::Popup,
    ui_tools::login_ui::render_login_form,
};
use ratatui::widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph};
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
    let previous_dir = List::new(["wk_,root dir"]).block(previous_dir_block);
    let current_dir =
        List::new(model.current_dir.iter().map(|f| to_list_item(f))).block(current_dir_block);
    let next_dir = List::new(["z", "z"]).block(next_dir_block);
    frame.render_widget(previous_dir, vertical_layouts[0]);
    frame.render_widget(current_dir, vertical_layouts[1]);
    frame.render_widget(next_dir, vertical_layouts[2]);

    match &model.popup {
        Some(Popup::Config) => {
            let config_text = get_text_config(model);
            let title = model.config_path.display().to_string();
            let block = Block::bordered().title(title);
            let content = Paragraph::new(config_text).block(block);
            let area = centered_rect(70, 50, frame.size());
            frame.render_widget(Clear, area); //this clears out the background
            frame.render_widget(content, area);
        }
        Some(Popup::LoginForm {
            code_input,
            // TODO render error
            error_message: _,
        }) => {
            render_login_form(model.config.api.auth_link(), code_input.clone(), frame);
        }
        None => (),
    };
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

fn to_list_item(file: &File) -> ListItem {
    if file.active {
        ListItem::new(file.name.clone()).black().bold().on_blue()
    } else {
        match file.file_type {
            NodeType::Dir => ListItem::new(file.name.clone()).on_black(),
            NodeType::File => ListItem::new(file.name.clone()).on_black().dim().blue(),
        }
    }
}
