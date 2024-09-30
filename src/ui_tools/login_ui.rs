use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::Frame;

use crate::ui::centered_rect;
use ratatui::widgets::{Block, Clear, Paragraph};
use ratatui::{
    layout::{Alignment, Flex},
    widgets::Padding,
};

pub fn render_login_form(auth_link: String, code_input: String, frame: &mut Frame) {
    let area = centered_rect(70, 50, frame.size());
    let [link_area] = Layout::vertical([Constraint::Length(4)]).areas(area);
    let [input_area_2] = Layout::vertical([Constraint::Length(4)])
        .flex(Flex::Center)
        .areas(area);

    let [input_h] = Layout::horizontal([Constraint::Length(15)])
        .flex(Flex::Center)
        .areas(input_area_2);

    let header = Paragraph::new("")
        .style(Style::default().fg(Color::Yellow))
        .block(Block::bordered().title("Please log in!"));
    let link_b = Block::new().padding(Padding::top(3));
    let label_p = Paragraph::new(format!(" Go to: {}", auth_link))
        .alignment(Alignment::Center)
        .block(link_b);
    let input_b = Block::bordered().title("Enter code:");
    let input_p = Paragraph::new(code_view(code_input))
        .centered()
        .block(input_b);

    frame.render_widget(Clear, area);
    frame.render_widget(header, area);
    frame.render_widget(label_p, link_area);
    frame.render_widget(input_p, input_h);
}

fn code_view(code_input: String) -> String {
    code_input
        .split("")
        .filter(|v| !v.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}
