use alloy::primitives::Address;
use foundry_block_explorers::account::{NormalTransaction, TxListParams};
use serde::{Deserialize, Serialize};
use t_lib::{
    extension::optional::Optional,
    log::{Level, instrument},
};

use crate::fetch::{
    Fetch, Param,
    etherscan::{
        client::EtherscanClient,
        error::{Error, Result},
        model::pagination::Pagination,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
    pub address: Address,
    pub start_block: u64,
    pub end_block: u64,
    #[serde(default, flatten)]
    pub pagination: Option<Pagination>,
}

impl Param for Params {
    type Err = Error;
    type Ret = Vec<NormalTransaction>;
}

impl Fetch<Params> for EtherscanClient {
    type Err = <Params as Param>::Err;
    type Ret = <Params as Param>::Ret;

    #[instrument(level = Level::TRACE, skip_all, err, fields(?params))]
    async fn fetch(&mut self, params: Params) -> Result<Self::Ret, Self::Err> {
        let Params { address, start_block, end_block, pagination } = params;

        let Pagination { page, offset, sort } = Optional::value(pagination);
        let tx_list_params = TxListParams { start_block, end_block, page, offset, sort };
        // TODO: FIX pagination limit
        let txs = self.get_transactions(&address, Some(tx_list_params)).await?;
        Ok(txs)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use alloy::primitives::TxHash;
    use nill::{Nil, nil};

    use super::*;
    use crate::fetch::etherscan::EtherscanFetch;

    const ADDRESS: &str = "0xcf4f5cbc40ab3c8d8b0bfe752f70bf0916c0d938";

    #[tokio::test]
    async fn test_get_transactions() -> Result<Nil> {
        let params = Params {
            address: ADDRESS.parse()?,
            start_block: 55911167,
            end_block: 56045281,
            pagination: None,
        };
        let txs = params.fetch().await?;

        let tx_hash = txs[0].hash.value().expect("Tx Hash");
        let hash = "0x8ad8001c29dc528395ee6055711ee00025cba5e0f4828396726f6d018565595f";

        assert_eq!(tx_hash, &TxHash::from_str(hash)?);
        assert_eq!(txs.len(), 52);
        Ok(nil)
    }
}
