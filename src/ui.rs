use crate::{
    config::get_text_config,
    model::{self, Popup, *},
};
use ratatui::style::palette::tailwind;
use ratatui::widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph};
use ratatui::Frame;
use ratatui::{
    layout::{Alignment, Flex, Position},
    prelude::Stylize,
    widgets::Padding,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
};

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
        Some(Popup::LoginForm { input }) => {
            let area = centered_rect(70, 50, frame.size());
            let [link_area] = Layout::vertical([Constraint::Length(4)]).areas(area);
            let [input_area_2] = Layout::vertical([Constraint::Length(4)])
                .flex(Flex::Center)
                .areas(area);

            let [input_h] = Layout::horizontal([Constraint::Length(20)])
                .flex(Flex::Center)
                .areas(input_area_2);

            let header = Paragraph::new("")
                .style(Style::default().fg(Color::Yellow))
                .block(Block::bordered().title("Please log in!"));
            let link_b = Block::new().padding(Padding::top(3));
            let label_p = Paragraph::new(format!(" Go to: {}", model.config.api.auth_link()))
                .alignment(Alignment::Center)
                .block(link_b);
            let input_b = Block::bordered().title("Enter code:");
            let input_p = Paragraph::new(input.as_str()).block(input_b);

            frame.render_widget(Clear, area);
            frame.render_widget(header, area);

            frame.render_widget(label_p, link_area);

            frame.render_widget(input_p, input_h);
            frame.set_cursor(input_h.x + 1, input_h.y + 1);
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
        ListItem::new(file.name.clone()).bg(tailwind::RED.c950)
    } else {
        ListItem::new(file.name.clone())
    }
}
