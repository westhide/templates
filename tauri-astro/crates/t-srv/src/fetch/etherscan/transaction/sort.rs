use foundry_block_explorers::account::Sort as TxListSort;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sort {
    Asc,
    Desc,
}

impl From<Sort> for TxListSort {
    fn from(sort: Sort) -> Self {
        match sort {
            Sort::Asc => Self::Asc,
            Sort::Desc => Self::Desc,
        }
    }
}
