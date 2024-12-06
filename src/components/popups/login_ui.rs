use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::Frame;
use rust_i18n::t;

use crate::ui::centered_rect;
use ratatui::widgets::{Block, Clear, Paragraph};
use ratatui::{
    layout::{Alignment, Flex},
    widgets::Padding,
};

pub fn render_login_form(
    frame: &mut Frame,
    auth_link: String,
    code_input: String,
    error_message: Option<String>,
) {
    let area = centered_rect(50, 60, frame.size());
    let [link_area, _space_1, input_area, _space_2, error_area] = Layout::vertical([
        Constraint::Length(4),
        Constraint::Length(8),
        Constraint::Length(3),
        Constraint::Length(2),
        Constraint::Length(4),
    ])
    .areas(area);

    let [input_h] = Layout::horizontal([Constraint::Length(40)])
        .flex(Flex::Center)
        .areas(input_area);

    let [error_h] = Layout::horizontal([Constraint::Length(60)])
        .flex(Flex::Center)
        .areas(error_area);

    let header = Paragraph::new("")
        .style(Style::default().fg(Color::Yellow))
        .block(Block::bordered().title(t!("login_form.please_log_in").to_string()));

    let link_b = Block::new().padding(Padding::top(3));
    let label_p = Paragraph::new(format!(" {}: {}", t!("login_form.go_to"), auth_link))
        .alignment(Alignment::Center)
        .block(link_b);

    let input_b = Block::bordered().title(t!("login_form.enter_code").to_string());
    let input_p = Paragraph::new(code_view(code_input))
        .centered()
        .block(input_b);

    frame.render_widget(Clear, area);
    frame.render_widget(header, area);
    frame.render_widget(label_p, link_area);
    frame.render_widget(input_p, input_h);

    if let Some(msg) = error_message {
        let error_b = Block::new();
        let error_p = Paragraph::new(msg.to_string()).centered().block(error_b);
        frame.render_widget(error_p, error_h);
    }
}

fn code_view(code_input: String) -> String {
    code_input
        .split("")
        .filter(|v| !v.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}
