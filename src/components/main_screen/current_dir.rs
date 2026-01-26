use crate::error::AppError;
use crate::models::file::File;
use crate::utils::common::AppErrorUnit;
use ratatui::layout::Rect;
use ratatui::prelude::{Modifier, Style};
use ratatui::style::palette::material::BLUE;
use ratatui::style::palette::tailwind::SLATE;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, HighlightSpacing, List, ListItem, ListState};
use ratatui::{symbols, Frame};
use std::path::PathBuf;

const HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

#[derive(Debug, Clone)]
pub struct CurrentDir {
    pub path: PathBuf,
    pub item: File,
    pub items: Vec<File>,
    pub state: ListState,
}

impl CurrentDir {
    pub fn new(path: PathBuf, item: File, items: Vec<File>) -> Self {
        Self {
            path,
            item,
            items,
            state: ListState::default().with_selected(Some(0)),
        }
    }

    pub fn update(&self, item: File, items: Vec<File>) -> Self {
        Self {
            path: self.path.clone(),
            item: item,
            items: items,
            state: self.state.clone(),
        }
    }

    pub fn render(&mut self, area: Rect, frame: &mut Frame) {
        let header: Block = Block::new()
            .title(Line::raw(self.path.to_string_lossy()).centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(HEADER_STYLE);

        let items: Vec<ListItem> = self.items.iter().map(ListItem::from).collect();

        let current_dirs_list = List::new(items)
            .block(header)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        frame.render_stateful_widget(current_dirs_list, area, &mut self.state)
    }

    pub fn selected_file(&self) -> Result<File, AppErrorUnit> {
        let list_item_state = self
            .state
            .selected()
            .ok_or(AppError::MissingSelectedElelement)?;

        let file = self
            .items
            .get(list_item_state)
            .cloned()
            .ok_or(AppError::MissingSelectedElelement);

        file
    }
}
