use ratatui::{layout::Rect, Frame};

pub trait Widget {
    fn render(&self, frame: &mut Frame, area: Rect);
}
