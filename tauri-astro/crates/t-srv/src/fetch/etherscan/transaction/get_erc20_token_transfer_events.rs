use foundry_block_explorers::account::{
    ERC20TokenTransferEvent, Sort, TokenQueryOption, TxListParams,
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
    pub contract: String,
    pub start_block: u64,
    pub end_block: u64,
    pub page: u64,
    pub offset: u64,
    pub sort: Sort,
}

impl Param for Params {
    type Err = Error;
    type Ret = Vec<ERC20TokenTransferEvent>;
}

impl Fetch<Params> for EtherscanClient {
    type Err = <Params as Param>::Err;
    type Ret = <Params as Param>::Ret;

    #[instrument(level = Level::TRACE, skip_all, err, fields(?params))]
    async fn fetch(&mut self, params: Params) -> Result<Self::Ret, Self::Err> {
        let Params { address, contract, start_block, end_block, page, offset, sort } = params;

        let address = address.parse()?;
        let contract_address = contract.parse()?;
        let option = TokenQueryOption::ByAddressAndContract(address, contract_address);
        let tx_list_params = TxListParams { start_block, end_block, page, offset, sort };
        let txs = client()?.get_erc20_token_transfer_events(option, Some(tx_list_params)).await?;
        Ok(txs)
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_yaml_snapshot;
    use nill::{Nil, nil};

    use super::*;
    use crate::fetch::etherscan::EtherscanFetch;

    const ADDRESS: &str = "0xcf4f5cbc40ab3c8d8b0bfe752f70bf0916c0d938";

    #[tokio::test]
    async fn test_get_transactions() -> Result<Nil> {
        const ULTI_TOKEN: &str = "0x0e7779e698052f8fe56c415c3818fcf89de9ac6d";
        let params = Params {
            address: ADDRESS.into(),
            contract: ULTI_TOKEN.into(),
            start_block: 55911167,
            end_block: 56045281,
            page: 0,
            offset: 1000,
            sort: Sort::Asc,
        };
        let txs = params.fetch().await?;

        assert_yaml_snapshot!(
            &txs[0],
            { ".confirmations" =>"[confirmations]",},
            @r#"
        blockNumber: "0x355441c"
        timeStamp: "1753937753"
        hash: "0xf04e017634abb40ed16e18b19459c4a968cdb6a56dd9888c63b15530cd57e91d"
        nonce: "0x259"
        blockHash: "0x30fbec7cac0953e252a8384a71c6caa3163630833bce29520fe63d909277358c"
        from: "0xcf4f5cbc40ab3c8d8b0bfe752f70bf0916c0d938"
        contractAddress: "0x0e7779e698052f8fe56c415c3818fcf89de9ac6d"
        to: "0xb300000b72deaeb607a12d5f54773d1c19c7028d"
        value: "0x8f8dc478c2d4ba5d2f00"
        tokenName: Ultiverse Token
        tokenSymbol: ULTI
        tokenDecimal: "18"
        transactionIndex: 70
        gas: "0x522c9"
        gasPrice: "0x68e7780"
        gasUsed: "0x2ab14"
        cumulativeGasUsed: "0x8a9791"
        input: deprecated
        confirmations: "[confirmations]"
        "#
        );
        Ok(nil)
    }
}
