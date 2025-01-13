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

#[derive(Debug, Clone)]
pub struct PreviousDir {
    pub path: PathBuf,
    pub item: File,
    pub items: Vec<File>,
}

impl ReaderHOF for PreviousDir {
    fn from_path(path: PathBuf, dir_reader: DirReader) -> Self {
        let (item, items) = dir_reader
            .read_local_with_cloud(path.to_str().unwrap())
            .unwrap();
        Self::new(path.clone(), item, items)
    }
}

impl PreviousDir {
    pub fn new(path: PathBuf, item: File, items: Vec<File>) -> Self {
        Self { path, item, items }
    }

    pub fn render(&self, area: Rect, frame: &mut Frame) {
        let header: Block = Block::new()
            .title(Line::raw(self.path.to_str().unwrap()).centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(HEADER_STYLE);

        let items: Vec<ListItem> = self.items.iter().map(ListItem::from).collect();

        let current_dirs_list = List::new(items)
            .block(header)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        frame.render_widget(current_dirs_list, area)
    }

    pub fn render_empty(area: Rect, root_dir_title: String, frame: &mut Frame) {
        let header: Block = Block::new()
            .title(Line::raw(root_dir_title))
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(HEADER_STYLE);

        frame.render_widget(header, area)
    }
}
