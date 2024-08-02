
#[derive(Debug, Default, Clone)]
pub struct File {
    pub name: String,
    pub active: bool,
}

#[derive(Debug, Default)]
pub struct Model {
    pub previous_dir: Vec<File>,
    pub current_dir: Vec<File>,
    pub sub_dir: Vec<File>,
    pub active_file_row_index: i32,
}
