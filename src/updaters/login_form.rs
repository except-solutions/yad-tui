use crate::model::{Model, Popup};

const LOGIN_INPUT_MAX_DIGITS: u8 = 7;

pub fn update_input(popup: Option<Popup>, code_number: char) -> Option<Popup> {
    update_form(
        popup,
        |input| code_number.is_ascii_digit() && input.len() < LOGIN_INPUT_MAX_DIGITS as usize,
        |input| format!("{0}{1}", input, code_number),
    )
}

pub fn remove_last_symbol(popup: Option<Popup>) -> Option<Popup> {
    update_form(
        popup,
        |input| !input.is_empty(),
        |input| input[0..input.len() - 1].to_string(),
    )
}

fn update_form(
    popup: Option<Popup>,
    cond: impl Fn(String) -> bool,
    mutator: impl Fn(String) -> String,
) -> Option<Popup> {
    popup.map(|p| match p {
        Popup::LoginForm { code_input } => {
            let is_valid_input = cond(code_input.clone());

            if is_valid_input {
                Popup::LoginForm {
                    code_input: mutator(code_input),
                }
            } else {
                Popup::LoginForm { code_input }
            }
        }
        any_popup => any_popup,
    })
}
