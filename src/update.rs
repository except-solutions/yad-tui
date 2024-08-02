use crate::model::{File, Model};
use crate::update::Message::{Continue, Exit, MoveDown, MoveUp};

#[derive(PartialEq)]
pub enum Message {
    Exit,
    Continue,
    MoveDown,
    MoveUp,
}

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        MoveDown => {
            update_active_file(
                model,
                |i, files_count| (i as usize) < files_count,
               |i| i + 1
            );
            Some(Continue)
        }
        MoveUp => {
            update_active_file(
                model,
                |i, _| i  >= 0,
                |i| i - 1
            );
            Some(Continue)
        },
        Continue => Some(Continue),
        Exit => None,
    }
}

fn update_active_file(mut model: &mut Model, cond: fn(i32, usize) -> bool, mutator: fn(i32) -> i32) {
    let current_dir_size = model.current_dir.len();
    let new_active_file_row_index_guess = mutator(model.active_file_row_index);

    if cond(new_active_file_row_index_guess, current_dir_size) {
        model.active_file_row_index = new_active_file_row_index_guess;
        model.current_dir = model
            .current_dir
            .iter()
            .enumerate()
            .map(|(i, file)| File {
                active: i == model.active_file_row_index as usize,
                ..file.clone()
            })
            .collect();
    };
}
