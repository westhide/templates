use foundry_block_explorers::account::{
    InternalTransaction, InternalTxQueryOption, Sort, TxListParams,
};
use t_lib::log::{Level, instrument};

use crate::fetch::{
    Fetch, Param,
    etherscan::{
        client::{EtherscanClient, client},
        error::{Error, Result},
    },
};

#[derive(Debug, Clone)]
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
    type Ret = Vec<InternalTransaction>;
}

impl Fetch<Params> for EtherscanClient {
    type Err = <Params as Param>::Err;
    type Ret = <Params as Param>::Ret;

    #[instrument(level = Level::TRACE, skip_all, err, fields(?params))]
    async fn fetch(&mut self, params: Params) -> Result<Self::Ret, Self::Err> {
        let Params { address, start_block, end_block, page, offset, sort } = params;

        let address = address.parse()?;
        let option = InternalTxQueryOption::ByAddress(address);
        let tx_list_params = TxListParams { start_block, end_block, page, offset, sort };
        // FIXME: This API return Maximum 10,000 records
        let txs = client()?.get_internal_transactions(option, Some(tx_list_params)).await?;
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
            address: ADDRESS.into(),
            start_block: 55511167,
            end_block: 56045281,
            page: 0,
            offset: 1000,
            sort: Sort::Asc,
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
