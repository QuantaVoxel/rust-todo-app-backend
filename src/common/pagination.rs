use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_per_page")]
    pub per_page: u64,
}

fn default_page() -> u64 {
    1
}

fn default_per_page() -> u64 {
    20
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}

impl PaginationQuery {
    pub fn offset(&self) -> u64 {
        if self.page == 0 {
            0
        } else {
            (self.page - 1) * self.limit()
        }
    }

    pub fn limit(&self) -> u64 {
        if self.per_page == 0 {
            20
        } else if self.per_page > 100 {
            100
        } else {
            self.per_page
        }
    }
}
