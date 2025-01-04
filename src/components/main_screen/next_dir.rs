use crate::fs::ReaderHOF;
use crate::models::file::File;
use crate::utils::dir_reader::DirReader;
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
pub struct NextDir {
    pub path: PathBuf,
    pub items: Vec<File>,
}

impl ReaderHOF for NextDir {
    fn from_path(path: &String, dir_reader: DirReader) -> Self {
        let mut path_buf = PathBuf::new();
        path_buf.push(path);
        let items = dir_reader.read(path);
        Self::new(path_buf, items.unwrap())
    }
}

impl NextDir {
    pub fn new(path: PathBuf, items: Vec<File>) -> Self {
        Self { path, items }
    }

    pub fn render(&mut self, area: Rect, frame: &mut Frame) -> () {
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

    pub fn render_empty(area: Rect, frame: &mut Frame) {
        let header: Block = Block::new()
            .title(Line::raw("Preview block"))
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(HEADER_STYLE);

        frame.render_widget(header, area)
    }
}
