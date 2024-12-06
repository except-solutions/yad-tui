use crate::{
    meta_db::Meta,
    models::model::{Model, Popup},
};

const LOGIN_INPUT_MAX_DIGITS: u16 = 999;

pub fn update_input(popup: Option<Popup>, code_number: char) -> Option<Popup> {
    update_form(
        popup,
        |input| input.len() < LOGIN_INPUT_MAX_DIGITS as usize,
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
        Popup::LoginForm {
            code_input,
            error_message: _,
        } => {
            let is_valid_input = cond(code_input.clone());

            if is_valid_input {
                Popup::LoginForm {
                    code_input: mutator(code_input),
                    error_message: None,
                }
            } else {
                Popup::LoginForm {
                    code_input,
                    error_message: None,
                }
            }
        }
        any_popup => any_popup,
    })
}

pub fn send_form(model: &mut Model, code: String) {
    match model.disk_client.auth(code.clone()) {
        Ok(auth_response) => {
            model
                .meta_db
                .tx(true)
                .and_then(|tx| {
                    tx.get_or_create_bucket("meta")
                        .map(|bucket| {
                            let meta = Meta {
                                api_token: Some(auth_response.access_token),
                            };
                            let data = serde_json::to_vec(&meta).unwrap();
                            let _ = bucket.put("meta", data);
                            meta
                        })
                        .map(|meta| {
                            let _ = tx.commit();
                            model.meta = meta;
                            model.popup = None;
                        })
                })
                .unwrap_or_else(|err| {
                    log::error!("{}", err);
                    model.popup = Some(Popup::LoginForm {
                        code_input: "".to_string(),
                        error_message: Some(format!("{} - {}", "Auth token store error", err)),
                    })
                });
        }
        Err(error_message) => {
            model.popup = Some(Popup::LoginForm {
                code_input: code.clone(),
                error_message: Some(error_message),
            })
        }
    };
}
