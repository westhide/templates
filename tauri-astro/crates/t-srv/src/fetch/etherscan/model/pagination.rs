use foundry_block_explorers::account::Sort;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct Pagination {
    pub page: u64,
    pub offset: u64,
    pub sort: Sort,
}

const DEFAULT_PAGINATION: Pagination = Pagination { page: 1, offset: 1000, sort: Sort::Desc };

impl Default for Pagination {
    fn default() -> Self {
        DEFAULT_PAGINATION
    }
}
