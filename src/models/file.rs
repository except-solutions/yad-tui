use ratatui::prelude::{Color, Line};
use ratatui::style::palette::material::GREEN;
use ratatui::style::palette::tailwind::SLATE;
use ratatui::widgets::ListItem;
use std::path::PathBuf;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum State {
    Local,
    Cloud,
    Synced,
    Syncing,
}

impl State {
    pub fn symbol(&self) -> String {
        match self {
            State::Local => "(L)".to_string(),
            State::Cloud => "(C)".to_string(),
            State::Synced => "(S)".to_string(),
            State::Syncing => "(SI)".to_string(),
        }
    }

    pub fn in_cloud(&self) -> bool {
        !matches!(&self, State::Local)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CloudFile {
    pub path: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct LocalFile {
    pub path: PathBuf,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct File {
    pub name: String,
    pub file_type: NodeType,
    pub state: State,
    pub cloud: Option<CloudFile>,
    pub local: Option<LocalFile>,
}

#[derive(Clone, Debug, Ord, Eq, PartialOrd, PartialEq, Hash)]
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
            NodeType::File => Line::styled(
                format!(" ! {} {}", value.state.symbol(), value.name),
                TEXT_FG_COLOR,
            ),
            NodeType::Dir => Line::styled(
                format!(" * {} {}", value.state.symbol(), value.name),
                COMPLETED_TEXT_FG_COLOR,
            ),
        };
        ListItem::new(line)
    }
}
