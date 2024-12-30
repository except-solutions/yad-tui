use ratatui::prelude::{Color, Line};
use ratatui::style::palette::material::GREEN;
use ratatui::style::palette::tailwind::SLATE;
use ratatui::widgets::ListItem;
use std::path::PathBuf;


#[derive(Debug)]
enum State {
    Local,
    Cloud,
    Synced,
    Syncing
}


#[derive(Debug)]
pub struct CloudFile {

}

#[derive(Debug)]
pub struct LocalFile {

}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub file_type: NodeType,
    pub path: PathBuf,
    pub state: State,
    pub cloud: Option<CloudFile>,
    pub local: Option<LocalFile>
}

#[derive(Clone, Debug, Ord, Eq, PartialOrd, PartialEq)]
pub enum NodeType {
    File,
    Dir,
}

impl File {
    pub fn is_dir(&self) -> bool {
        self.file_type == NodeType::Dir
    }
    pub fn is_file(&self) -> bool {
        self.file_type == NodeType::File
    }
}

const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

impl From<&File> for ListItem<'_> {
    fn from(value: &File) -> Self {
        let line = match value.file_type {
            NodeType::File => Line::styled(format!(" ! {}", value.name), TEXT_FG_COLOR),
            NodeType::Dir => Line::styled(format!(" * {}", value.name), COMPLETED_TEXT_FG_COLOR),
        };
        ListItem::new(line)
    }
}

