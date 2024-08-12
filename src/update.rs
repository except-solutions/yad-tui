use crate::model::Model;
use crate::update::Message::{Continue, Exit, MoveDown, MoveUp, ShowConfig};

#[derive(PartialEq)]
pub enum Message {
    Exit,
    Continue,
    MoveDown,
    MoveUp,
    ShowConfig,
}

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        MoveDown => {
            update_active_file(
                model,
                |i, files_count| (i as usize) < files_count,
                |i| i + 1,
            );
            Some(Continue)
        }
        MoveUp => {
            update_active_file(model, |i, _| i >= 0, |i| i - 1);
            Some(Continue)
        }
        ShowConfig => {
            model.popup.show_config = !model.popup.show_config;
            Some(Continue)
        }
        Continue => Some(Continue),
        Exit => None,
    }
}

fn update_active_file(model: &mut Model, cond: fn(i32, usize) -> bool, mutator: fn(i32) -> i32) {
    let current_dir_size = model.current_dir.len();
    let new_active_file_row_index_guess = mutator(model.active_file_row_index);

    if cond(new_active_file_row_index_guess, current_dir_size) {
        model.set_active_file(new_active_file_row_index_guess)
    };
}
