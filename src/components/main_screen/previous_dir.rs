use crate::fs::ReaderHOF;
use crate::models::file::File;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Modifier, Style};
use ratatui::style::palette::material::BLUE;
use ratatui::style::palette::tailwind::SLATE;
use ratatui::widgets::{Block, Borders, HighlightSpacing, List, ListItem};
use ratatui::{symbols, Frame};
use std::path::PathBuf;

const HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

#[derive(Debug)]
pub struct PreviousDir {
    pub path: PathBuf,
    pub items: Vec<File>,
}

impl ReaderHOF for PreviousDir {
    fn from_path(path: &String, fs_reader: fn(&PathBuf) -> Vec<File>) -> Self {
        let mut current_buf = PathBuf::new();
        current_buf.push(path);

        let mut path_buf = PathBuf::new();

        if let Some(dir) = current_buf.parent() {
            path_buf.push(dir)
        } else {
            path_buf.push(path);
        }

        let items = fs_reader(&path_buf);
        Self::new(path_buf, items)
    }
}

impl PreviousDir {
    pub fn new(path: PathBuf, items: Vec<File>) -> Self {
        Self { items, path }
    }

    pub fn render(&self, area: Rect, frame: &mut Frame) -> () {
        let header: Block = Block::new()
            .title(Line::raw(self.path.to_string_lossy()).centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(HEADER_STYLE);

        let items: Vec<ListItem> = self.items.iter().map(|file| ListItem::from(file)).collect();

        let current_dirs_list = List::new(items)
            .block(header)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        frame.render_widget(current_dirs_list, area)
    }
}
