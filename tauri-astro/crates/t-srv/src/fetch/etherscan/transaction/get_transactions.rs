use foundry_block_explorers::account::{NormalTransaction, TxListParams};
use serde::{Deserialize, Serialize};
use t_lib::log::{Level, instrument};

use crate::fetch::{
    Fetch, Param,
    etherscan::{
        client::{EtherscanClient, client},
        error::{Error, Result},
        transaction::sort::Sort,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub address: String,
    pub start_block: u64,
    pub end_block: u64,
    pub page: u64,
    pub offset: u64,
    pub sort: Sort,
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
        let Params { address, start_block, end_block, page, offset, sort } = params;

        let address = address.parse()?;
        let sort = sort.into();
        let tx_list_params = TxListParams { start_block, end_block, page, offset, sort };
        // TODO: FIX pagination limit
        let txs = client()?.get_transactions(&address, Some(tx_list_params)).await?;
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
            address: ADDRESS.into(),
            start_block: 55911167,
            end_block: 56045281,
            page: 0,
            offset: 1000,
            sort: Sort::Asc,
        };
        let txs = params.fetch().await?;

        let tx_hash = txs[0].hash.value().unwrap();
        let hash = "0xf04e017634abb40ed16e18b19459c4a968cdb6a56dd9888c63b15530cd57e91d";

        assert_eq!(tx_hash, &TxHash::from_str(hash)?);
        assert_eq!(txs.len(), 52);
        Ok(nil)
    }
}
