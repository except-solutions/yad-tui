use yad_tui::model::{File, Model, NodeType};

#[test]
fn test_wrong_indexes() {
    let mut m = test_model();
    m.set_active_file(-1);
    assert_eq!(m.active_file_row_index, 0)
}


fn test_model() -> Model {

    let previous = vec![File {
        name: String::from("abc"),
        active: true,
        file_type: NodeType::File
    }];
    let next = vec![File {
        name: String::from("abc"),
        active: true,
        file_type: NodeType::File
    }];
    let current_dir = vec![
        File {
            name: String::from("abc"),
            active: true,
            file_type: NodeType::Dir
        },
        File {
            name: String::from("def"),
            active: false,
            file_type: NodeType::File
        },
        File {
            name: String::from("xyz"),
            active: false,
            file_type: NodeType::File
        },
        File {
            name: String::from("abc"),
            active: false,
            file_type: NodeType::Dir
        },
    ];

    Model {
        active_file_row_index: 0,
        previous_dir: previous,
        current_dir,
        sub_dir: next,
    }
}