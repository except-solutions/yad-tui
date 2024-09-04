use yad_tui::{
    model::Popup,
    updaters::login_form::{remove_last_symbol, update_input},
};

#[test]
fn test_add_symbol() {
    let test_popup = Popup::LoginForm {
        code_input: "".to_string(),
    };

    match update_input(Some(test_popup), '1') {
        Some(Popup::LoginForm { code_input }) => {
            assert_eq!(code_input, "1".to_string(), "should be changed")
        }

        _ => panic!("wrong_popup type"),
    }
}

#[test]
fn test_add_wrong_symbol() {
    let test_popup = Popup::LoginForm {
        code_input: "".to_string(),
    };

    match update_input(Some(test_popup), 'a') {
        Some(Popup::LoginForm { code_input }) => assert_eq!(
            code_input,
            "".to_string(),
            "should not change on wrong symbol input"
        ),

        _ => panic!("wrong_popup type"),
    }
}

#[test]
fn test_delete_symbol() {
    let test_popup = Popup::LoginForm {
        code_input: "123".to_string(),
    };

    match remove_last_symbol(Some(test_popup)) {
        Some(Popup::LoginForm { code_input }) => {
            assert_eq!(code_input, "12".to_string(), "should remove last symbol")
        }

        _ => panic!("wrong_popup type"),
    }
}
