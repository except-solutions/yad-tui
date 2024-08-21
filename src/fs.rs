use crate::model::{File, NodeType};
use std::fs;

pub fn get_init_fs_tree(init_path: &Option<&String>) -> Vec<File> {
    let config_path = init_path
        .expect("Can`t read `sync_dir_path` from conf. Check conf `.config/yad/Config.toml`");

    match fs::read_dir(config_path) {
        Ok(entries) => {
            let (results, errors): (Vec<_>, Vec<_>) = entries
                .enumerate()
                .map(|(i, dir_entry)| match dir_entry {
                    Ok(dir_entry) => match dir_entry.file_type() {
                        Ok(file_type) => Ok(File {
                            name: dir_entry.file_name().into_string().unwrap(),
                            active: i == 0,
                            file_type: if file_type.is_file() {
                                NodeType::File
                            } else {
                                NodeType::Dir
                            },
                        }),
                        Err(err) => Err(format!("Unexpected file type. Original exc: {err}")),
                    },
                    Err(err) => Err(format!(
                        "Unexpected entity in file system. Original exc: {err}"
                    )),
                })
                .partition(Result::is_ok);

            let mut results: Vec<File> = results.into_iter().map(Result::unwrap).collect();
            results.sort_by(|current, next| next.file_type.cmp(&current.file_type));

            let _errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
            results
        }
        Err(err) => {
            panic!("Can`t read `sync_dir_path` from conf. Check conf `.config/yad/Config.toml`. Original exc: {err}", err=err)
        }
    }
}
