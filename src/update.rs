use crate::model::{Model, Popup::*};
use crate::update::Message::{Continue, Exit, MoveDown, MoveUp, ShowConfig};
use crate::updaters::login_form::{remove_last_symbol, send_form, update_input};

#[derive(PartialEq)]
pub enum InputAction {
    InputChar(char),
    DeleteChar,
    Send,
}

#[derive(PartialEq)]
pub enum Message {
    Exit,
    Continue,
    MoveDown,
    MoveUp,
    ShowConfig,
    ClosePopup,
    InputModeAction(InputAction),
}

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match (msg, model.popup.clone()) {
        (MoveDown, None) => {
            update_active_file(
                model,
                |i, files_count| (i as usize) < files_count,
                |i| i + 1,
            );
            Some(Continue)
        }
        (MoveUp, None) => {
            update_active_file(model, |i, _| i >= 0, |i| i - 1);
            Some(Continue)
        }
        (ShowConfig, None) => {
            model.popup = Some(Config);
            Some(Continue)
        }
        (Continue, _) => Some(Continue),
        (Exit, _) => None,
        (Message::ClosePopup, _) => {
            model.popup = None;
            Some(Continue)
        }
        (
            Message::InputModeAction(InputAction::InputChar(code_number)),
            Some(LoginForm {
                code_input,
                error_message,
            }),
        ) => {
            model.popup = update_input(model.popup.clone(), code_number);
            Some(Continue)
        }
        (Message::InputModeAction(InputAction::DeleteChar), LoginForm) => {
            model.popup = remove_last_symbol(model.popup.clone());
            Some(Continue)
        }
        (
            Message::InputModeAction(InputAction::Send),
            Some(LoginForm {
                code_input,
                error_message,
            }),
        ) => {
            send_form(model, code_input);
            Some(Continue)
        }
        (_, _) => Some(Continue),
    }
}

fn update_active_file(model: &mut Model, cond: fn(i32, usize) -> bool, mutator: fn(i32) -> i32) {
    let current_dir_size = model.current_dir.len();
    let new_active_file_row_index_guess = mutator(model.active_file_row_index);

    if cond(new_active_file_row_index_guess, current_dir_size) {
        match model.set_active_file(new_active_file_row_index_guess) {
            Ok(()) => (),
            Err(m) => println!("){}", m),
        }
    };
}
