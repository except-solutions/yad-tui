use crate::{
    disk_client::DiskMetaResponse,
    utils::common::bytes_to_gbytes,
};

#[derive(Debug, Clone)]
pub struct DiskMeta {
    used_space: u64,
    total_space: u64,
    pub username: String,
}

impl From<DiskMetaResponse> for DiskMeta {
    fn from(response: DiskMetaResponse) -> DiskMeta {
        DiskMeta {
            used_space: response.used_space,
            total_space: response.total_space,
            username: response.user.display_name,
        }
    }
}

impl DiskMeta {
    pub fn new(
        used_space: u64,
        total_space: u64,
        username: impl Into<String>,
    ) -> Self {
        DiskMeta {
            used_space,
            total_space,
            username: username.into(),
        }
    }

    pub fn used_space(&self) -> u64 {
        self.used_space
    }

    pub fn total_space(&self) -> u64 {
        self.total_space
    }

    pub fn free_space(&self) -> u64 {
        self.total_space.saturating_sub(self.used_space)
    }

    pub fn used_space_verbose(&self) -> String {
        Self::space_verbose(self.used_space)
    }

    pub fn total_space_verbose(&self) -> String {
        Self::space_verbose(self.total_space)
    }

    pub fn free_space_verbose(&self) -> String {
        Self::space_verbose(self.free_space())
    }

    pub fn usage_ratio(&self) -> f64 {
        if self.total_space == 0 {
            0.0
        } else {
            self.used_space as f64 / self.total_space as f64
        }
    }

    fn space_verbose(space: u64) -> String {
        format!("{:.2} GB", bytes_to_gbytes(space))
    }
}
