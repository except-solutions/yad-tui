use ratatui::{layout::{Constraint, Layout, Rect}, widgets::{Block, Paragraph}, Frame};

use crate::{components::common::Widget, models::{disk::DiskMeta, model::Model}};



#[derive(Debug, Clone)]
pub struct TopBar {
    pub disk_meta: DiskMeta
}

impl Widget for TopBar {

    fn render(&self, frame: &mut Frame, area: Rect) {
        let [user_a, total_space_a, used_space_a] = Layout::horizontal([
            Constraint::Length(20),
            Constraint::Length(15),
            Constraint::Length(15)
        ]).areas(area);

        let user_p = Paragraph::new(format!("Hello, {}", &self.disk_meta.username));
        let total_space_p = Paragraph::new(format!("tspace {}", &self.disk_meta.total_space));
        let used_space_p = Paragraph::new(format!("uspace {}", &self.disk_meta.used_space));
        

        frame.render_widget(user_p, user_a);
        frame.render_widget(total_space_p, total_space_a);
        frame.render_widget(used_space_p, used_space_a);
    }
}

impl DiskMeta {

}

