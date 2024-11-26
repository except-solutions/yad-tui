use crate::structs::file::{File, NodeType};
use ratatui::prelude::Color;
use ratatui::style::palette::material::GREEN;
use ratatui::style::palette::tailwind::SLATE;
use ratatui::text::Line;
use ratatui::widgets::{ListItem, ListState};

const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

#[derive(Debug)]
pub struct CurrentDirs {
    pub items: Vec<File>,
    pub state: ListState,
}

impl From<&File> for ListItem<'_> {
    fn from(value: &File) -> Self {
        let line = match value.file_type {
            NodeType::File => Line::styled(format!(" ! {}", value.name), TEXT_FG_COLOR),
            NodeType::Dir => Line::styled(format!(" * {}", value.name), COMPLETED_TEXT_FG_COLOR),
        };
        ListItem::new(line)
    }
}
