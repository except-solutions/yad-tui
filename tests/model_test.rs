use yad_tui::model::{File, Model, NodeType};

#[test]
fn test_wrong_indexes() {
    let mut m = setup();
    let too_small_index = -1;
    let too_big_index = (m.current_dir.len() + 1) as i32;
    assert_eq!(
        m.set_active_file(too_small_index),
        Err(format!("Inavlid index {too_small_index}"))
    );
    assert_eq!(
        m.set_active_file(too_big_index),
        Err(format!("Inavlid index {too_big_index}"))
    );
}

fn setup() -> Model {
    let previous = vec![File {
        name: String::from("abc"),
        active: true,
        file_type: NodeType::File,
    }];
    let next = vec![File {
        name: String::from("abc"),
        active: true,
        file_type: NodeType::File,
    }];
    let current_dir = vec![
        File {
            name: String::from("abc"),
            active: true,
            file_type: NodeType::Dir,
        },
        File {
            name: String::from("def"),
            active: false,
            file_type: NodeType::File,
        },
        File {
            name: String::from("xyz"),
            active: false,
            file_type: NodeType::File,
        },
        File {
            name: String::from("abc"),
            active: false,
            file_type: NodeType::Dir,
        },
    ];

    Model {
        active_file_row_index: 0,
        popup: Default::default(),
        config: Default::default(),
        previous_dir: previous,
        current_dir,
        sub_dir: next,
        config_path: Default::default(),
    }
}
