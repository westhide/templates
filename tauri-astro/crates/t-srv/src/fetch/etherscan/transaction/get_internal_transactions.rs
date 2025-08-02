use alloy::primitives::Address;
use foundry_block_explorers::account::{InternalTransaction, InternalTxQueryOption, TxListParams};
use serde::{Deserialize, Serialize};
use t_lib::{
    log::{Level, instrument},
    share::optional::Optional,
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
    type Ret = Vec<InternalTransaction>;
}

impl Fetch<Params> for EtherscanClient {
    type Err = <Params as Param>::Err;
    type Ret = <Params as Param>::Ret;

    #[instrument(level = Level::TRACE, skip_all, err, fields(?params))]
    async fn fetch(&mut self, params: Params) -> Result<Self::Ret, Self::Err> {
        let Params { address, start_block, end_block, pagination } = params;

        let option = InternalTxQueryOption::ByAddress(address);
        let Pagination { page, offset, sort } = Optional::value(pagination);
        let tx_list_params = TxListParams { start_block, end_block, page, offset, sort };
        // FIXME: This API return Maximum 10,000 records
        let txs = self.get_internal_transactions(option, Some(tx_list_params)).await?;
        Ok(txs)
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use nill::{Nil, nil};

    use super::*;
    use crate::fetch::etherscan::EtherscanFetch;

    const ADDRESS: &str = "0xcf4f5cbc40ab3c8d8b0bfe752f70bf0916c0d938";

    #[tokio::test]
    async fn test_get_transactions() -> Result<Nil> {
        let params = Params {
            address: ADDRESS.parse()?,
            start_block: 55511167,
            end_block: 56045281,
            pagination: None,
        };
        let txs = params.fetch().await?;

        assert_debug_snapshot!(txs, @r#"
        [
            InternalTransaction {
                block_number: Number(
                    55855906,
                ),
                time_stamp: "1753889945",
                hash: 0x555a56752ab2cb82a6581c57008f4c58340b5feccdee8e2eedf5ef9027b56b80,
                from: 0xb300000b72deaeb607a12d5f54773d1c19c7028d,
                to: Some(
                    0xcf4f5cbc40ab3c8d8b0bfe752f70bf0916c0d938,
                ),
                value: 12645685986925074,
                contract_address: None,
                input: None,
                result_type: "call",
                gas: 285814,
                gas_used: 0,
                trace_id: "0_1_1",
                is_error: "0",
                err_code: "",
            },
        ]
        "#);
        Ok(nil)
    }
}
