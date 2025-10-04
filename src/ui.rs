use crate::{
    components::{
        common::Widget, main_screen::previous_dir::PreviousDir, popups::login_ui::render_login_form,
    },
    config::get_text_config,
    disk_client::DiskClientT,
    models::model::{Model, Popup},
};
use log::warn;
use ratatui::layout::{Constraint, Layout, Rect};

use crate::components::main_screen::next_dir::NextDir;
use ratatui::widgets::{Block, Clear, Paragraph};
use ratatui::Frame;

pub fn ui<T: DiskClientT>(model: &mut Model<T>, frame: &mut Frame) {
    if model.is_authenticated() {
        let [top_bar, dirs] =
            Layout::vertical([Constraint::Percentage(5), Constraint::Percentage(95)])
                .split(frame.size())
                .to_vec()[..]
        else {
            panic!("Unexpected areas count!");
        };

        let [previous_dir_area, current_dir_area, next_dir_area] = Layout::horizontal([
            Constraint::Percentage(20),
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .areas(dirs);

        if let Some(top_bar_widget) = &model.top_bar {
            top_bar_widget.render(frame, top_bar);
        };

        if let Some(ref previous_dir) = model.fs.previous_dir {
            previous_dir.render(previous_dir_area, frame);
        } else {
            PreviousDir::render_empty(
                previous_dir_area,
                model.config.main.sync_dir_path.clone(),
                frame,
            );
        }

        model.fs.current_dir.render(current_dir_area, frame);

        if let Some(ref mut next_dir) = model.fs.next_dir {
            next_dir.render(next_dir_area, frame);
        } else {
            NextDir::render_empty(next_dir_area, frame)
        }
    }

    match (model.is_authenticated(), model.popup.clone()) {
        (true, Some(Popup::Config)) => {
            let config_text = get_text_config(model);
            let title = model.config_path.display().to_string();
            let block = Block::bordered().title(title);
            let content = Paragraph::new(config_text).block(block);
            let area = centered_rect(70, 50, frame.size());
            frame.render_widget(Clear, area); //this clears out the background
            frame.render_widget(content, area);
        }
        (
            false,
            Some(Popup::LoginForm {
                code_input,
                error_message,
            }),
        ) => {
            render_login_form(
                frame,
                model.config.api.auth_link(),
                code_input.clone(),
                error_message.clone(),
            );
        }
        (true, None) => (),
        invalid_state => warn!("Invalid ui state: {invalid_state:?}"),
    }
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
