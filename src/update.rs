use crate::channels::Channels;
use crate::models::model::{Model, Popup};
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
    EnterSelectedDir,
    EnterPrevDir,
    InputModeAction(InputAction),
    DownloadFile
}

pub fn update(model: &mut Model, msg: Message, channels: &Channels) -> Option<Message> {
    match (msg, model.popup.clone()) {
        (MoveDown, None) => {
            model
                .fs
                .select_next_element_for_next_dir(channels.read_next_dir_ch.sender.clone())
                .unwrap();
            Some(Continue)
        }
        (MoveUp, None) => {
            model
                .fs
                .select_previous_element_for_next_dir(channels.read_next_dir_ch.sender.clone())
                .unwrap();
            Some(Continue)
        }
        (ShowConfig, None) => {
            model.popup = Some(Popup::Config);
            Some(Continue)
        }
        (Message::EnterSelectedDir, None) => {
            model.fs.open_selected().unwrap();
            Some(Continue)
        }
        (Message::EnterPrevDir, None) => {
            model.fs.open_previous().unwrap();
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
            Some(Popup::LoginForm {
                code_input: _,
                error_message: _,
            }),
        ) => {
            model.popup = update_input(model.popup.clone(), code_number);
            Some(Continue)
        }
        (Message::InputModeAction(InputAction::DeleteChar), _login_form) => {
            model.popup = remove_last_symbol(model.popup.clone());
            Some(Continue)
        }
        (
            Message::InputModeAction(InputAction::Send),
            Some(Popup::LoginForm {
                code_input,
                error_message: _,
            }),
        ) => {
            send_form(model, code_input);
            Some(Continue)
        }
        (Message::DownloadFile, _) => {
            model.fs.download_selected();
            Some(Continue)
        }
        (_, _) => Some(Continue),
    }
}
