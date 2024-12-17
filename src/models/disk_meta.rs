use crate::{disk_client::DiskMetaResponse, utils::common::{bytes_to_gbytes, slice_string_by_max_value}}; #[derive(Debug, Clone)] pub struct DiskMeta { used_space: u64, total_space: u64, pub username: String }

impl From<DiskMetaResponse> for DiskMeta {
    fn from(disk_meta_response: DiskMetaResponse) -> DiskMeta {
        DiskMeta {
            used_space: disk_meta_response.used_space,
            total_space: disk_meta_response.total_space,
            username: disk_meta_response.user.display_name
        }
    }
}

impl DiskMeta {
    const DISK_SPACE_SYMBOLS_COUNT: u8 = 5;

    pub fn used_space_verbose(&self) -> String {
        self.space_vervose(self.used_space)
    }

    pub fn total_space_verbose(&self) -> String {
       self.space_vervose(self.total_space)
    }
    
    fn space_vervose(&self, space: u64) -> String {
        let space = slice_string_by_max_value(bytes_to_gbytes(space).to_string(), Self::DISK_SPACE_SYMBOLS_COUNT as usize);

        space + " GB"
    }
}

