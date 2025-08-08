use foundry_block_explorers::block_number::BlockNumber;
use serde::{Deserialize, Serialize};
use t_lib::log::{Level, instrument};

use crate::{
    error::{Error, err},
    fetch::{Fetch, Param, etherscan::client::Etherscan},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
    pub ts: u64,
    pub closest: Closest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Closest {
    Prev,
    Next,
}

impl Closest {
    pub fn closest(&self) -> &str {
        match self {
            Self::Prev => "before",
            Self::Next => "after",
        }
    }
}

impl Params {
    pub fn new(ts: u64, closest: Closest) -> Self {
        Self { ts, closest }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub block_number: usize,
}

impl Param for Params {
    type Err = Error;
    type Ret = u64;
}

impl Fetch<Params> for Etherscan {
    type Err = <Params as Param>::Err;
    type Ret = <Params as Param>::Ret;

    #[instrument(level = Level::TRACE, skip_all, err, fields(?params))]
    async fn fetch(&mut self, params: Params) -> Result<Self::Ret, Self::Err> {
        let Params { ts, closest } = params;
        let closest = closest.closest();
        let block = self.get_block_by_timestamp(ts, closest).await?;
        match block.block_number {
            BlockNumber::Number(num) => Ok(num.to()),
            block_number => err!("BlockNumber isn't number: {block_number}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use nill::{Nil, nil};

    use super::*;
    use crate::fetch::etherscan::EtherscanFetch;

    #[tokio::test]
    async fn test_get_block_number() -> Result<Nil, Error> {
        let param = Params { ts: 1754024487, closest: Closest::Prev };
        let block_number = param.fetch().await?;

        assert_eq!(block_number, 56035281);
        Ok(nil)
    }
}
