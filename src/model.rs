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

impl Model {
    pub fn set_active_file(&mut self, new_pos_index: i32) {
        self.active_file_row_index = new_pos_index;
        self.current_dir = self
            .current_dir
            .iter()
            .enumerate()
            .map(|(i, file)| File {
                active: i == new_pos_index as usize,
                ..file.clone()
            })
            .collect();
    }
}
