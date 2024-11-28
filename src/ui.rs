use crate::config::get_text_config;
use crate::model::*;
use crate::{model::Popup, ui_tools::login_ui::render_login_form};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::palette::material::BLUE;
use ratatui::style::palette::tailwind::SLATE;
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{
    Block, BorderType, Borders, Clear, HighlightSpacing, List, ListItem, Paragraph,
};
use ratatui::{symbols, Frame};

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

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

    let next_dir_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let previous_dir = List::new(["wk_,root dir"]).block(previous_dir_block);

    let next_dir = List::new(["z", "z"]).block(next_dir_block);

    let block = Block::new()
        .title(Line::raw("Current dir").centered())
        .borders(Borders::TOP)
        .border_set(symbols::border::EMPTY)
        .border_style(TODO_HEADER_STYLE);

    let current_dirs_items: Vec<ListItem> = model
        .current_dirs
        .items
        .iter()
        .map(|file| ListItem::from(file))
        .collect();

    let current_dirs_list = List::new(current_dirs_items)
        .block(block)
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_widget(previous_dir, vertical_layouts[0]);
    frame.render_stateful_widget(
        current_dirs_list,
        vertical_layouts[1],
        &mut model.current_dirs.state,
    );
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
            error_message,
        }) => {
            render_login_form(
                frame,
                model.config.api.auth_link(),
                code_input.clone(),
                error_message.clone(),
            );
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
