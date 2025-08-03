use alloy::primitives::Address;
use foundry_block_explorers::account::{ERC20TokenTransferEvent, TokenQueryOption, TxListParams};
use serde::{Deserialize, Serialize};
use t_lib::{
    log::{Level, instrument},
    share::optional::DefaultOption,
};

use crate::{
    error::Error,
    fetch::{
        Fetch, Param,
        etherscan::{client::Etherscan, model::pagination::Pagination},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
    pub address: Address,
    pub contract: Address,
    pub start_block: u64,
    pub end_block: u64,
    #[serde(default, flatten)]
    pub pagination: Option<Pagination>,
}

impl Param for Params {
    type Err = Error;
    type Ret = Vec<ERC20TokenTransferEvent>;
}

impl Fetch<Params> for Etherscan {
    type Err = <Params as Param>::Err;
    type Ret = <Params as Param>::Ret;

    #[instrument(level = Level::TRACE, skip_all, err, fields(?params))]
    async fn fetch(&mut self, params: Params) -> Result<Self::Ret, Self::Err> {
        let Params { address, contract, start_block, end_block, pagination } = params;

        let option = TokenQueryOption::ByAddressAndContract(address, contract);
        let Pagination { page, offset, sort } = DefaultOption::into(pagination);
        let tx_list_params = TxListParams { start_block, end_block, page, offset, sort };
        let txs = self.get_erc20_token_transfer_events(option, Some(tx_list_params)).await?;
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
    const ULTI_TOKEN: &str = "0x0e7779e698052f8fe56c415c3818fcf89de9ac6d";

    #[tokio::test]
    async fn test_get_transactions() -> Result<Nil, Error> {
        let params = Params {
            address: ADDRESS.parse()?,
            contract: ULTI_TOKEN.parse()?,
            start_block: 55911167,
            end_block: 56045281,
            pagination: None,
        };
        let txs = params.fetch().await?;

        assert_yaml_snapshot!(
            &txs[0],
            { ".confirmations" => "[confirmations]", },
            @r#"
        blockNumber: "0x3554d16"
        timeStamp: "1753939477"
        hash: "0xcc55871b93acc94dd0479fe0df3305b1f11153b4b80211a8981d067e4645705e"
        nonce: "0x28b"
        blockHash: "0xa426b3caf9e31d494bfac64f8f01d5a93b79cdd2189e2d83c97652d94e7b4169"
        from: "0xcf4f5cbc40ab3c8d8b0bfe752f70bf0916c0d938"
        contractAddress: "0x0e7779e698052f8fe56c415c3818fcf89de9ac6d"
        to: "0xb300000b72deaeb607a12d5f54773d1c19c7028d"
        value: "0x967f24f7aa50c6c55927"
        tokenName: Ultiverse Token
        tokenSymbol: ULTI
        tokenDecimal: "18"
        transactionIndex: 54
        gas: "0x588b6"
        gasPrice: "0x68e7780"
        gasUsed: "0x2e097"
        cumulativeGasUsed: "0x770202"
        input: deprecated
        confirmations: "[confirmations]"
        "#
        );
        assert_eq!(txs.len(), 52);
        Ok(nil)
    }
}
