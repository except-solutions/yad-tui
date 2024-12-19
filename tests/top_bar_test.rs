use yad_tui::{
    disk_client::{DiskMetaResponse, User},
    models::disk_meta::DiskMeta,
};

#[test]
fn test_top_bar_data_view() {
    let dmr = DiskMetaResponse {
        used_space: 89212034421,
        total_space: 1073741824000,
        user: User {
            display_name: "testuser".to_string(),
        },
    };

    let dm = DiskMeta::from(dmr);

    assert_eq!("83.09 GB".to_string(), dm.used_space_verbose());
    assert_eq!("1000.00 GB".to_string(), dm.total_space_verbose());
    assert_eq!("testuser".to_string(), dm.username);
}
