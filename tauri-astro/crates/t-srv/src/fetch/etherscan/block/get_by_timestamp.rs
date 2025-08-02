use foundry_block_explorers::block_number::BlockNumber;
use serde::{Deserialize, Serialize};
use t_lib::log::{Level, instrument};

use crate::fetch::{
    Fetch, Param,
    etherscan::{
        client::EtherscanClient,
        error::{Error, err},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub block_number: usize,
}

impl Param for Params {
    type Err = Error;
    type Ret = u64;
}

impl Fetch<Params> for EtherscanClient {
    type Err = <Params as Param>::Err;
    type Ret = <Params as Param>::Ret;

    #[instrument(level = Level::TRACE, skip_all, err, fields(?params))]
    async fn fetch(&mut self, params: Params) -> Result<Self::Ret, Self::Err> {
        let Params { timestamp } = params;
        let block = self.get_block_by_timestamp(timestamp, "before").await?;
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
    async fn test_get_block_by_timestamp() -> Result<Nil, Error> {
        let param = Params { timestamp: 1754024487 };
        let block_number = param.fetch().await?;

        assert_eq!(block_number, 56035281);
        Ok(nil)
    }
}
