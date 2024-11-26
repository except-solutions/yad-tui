use crate::{
    components::popups::login_ui::render_login_form,
    config::get_text_config,
    models::model::{Model, Popup},
};
use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::components::main_screen::next_dir::NextDir;
use ratatui::widgets::{Block, Clear, Paragraph};
use ratatui::Frame;

pub fn ui(model: &mut Model, frame: &mut Frame) {
    let [previous_dir_area, current_dir_area, next_dir_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(frame.size())
        .to_vec()[..]
    else {
        panic!("Unexpected areas")
    };

    model.fs.previous_dir.render(previous_dir_area, frame);
    model.fs.current_dir.render(current_dir_area, frame);

    if let Some(ref mut next_dir) = model.fs.next_dir {
        next_dir.render(next_dir_area, frame);
    } else {
        NextDir::render_empty(next_dir_area, frame)
    }

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
