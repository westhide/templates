use foundry_block_explorers::account::{
    ERC20TokenTransferEvent, InternalTransaction, InternalTxQueryOption, NormalTransaction,
    TokenQueryOption, TxListParams,
};
use t_lib::log::{Level, instrument};

use crate::api::etherscan::{client::client, error::Result};

#[instrument(level = Level::TRACE, err)]
pub async fn get_transactions(addr: &str, params: TxListParams) -> Result<Vec<NormalTransaction>> {
    let address = addr.parse()?;
    // TODO: FIX pagination limit
    let txs = client()?.get_transactions(&address, Some(params)).await?;
    Ok(txs)
}

#[instrument(level = Level::TRACE, err)]
pub async fn get_internal_transactions(
    addr: &str,
    params: TxListParams,
) -> Result<Vec<InternalTransaction>> {
    let address = addr.parse()?;
    let option = InternalTxQueryOption::ByAddress(address);
    // FIXME: This API return Maximum 10,000 records
    let txs = client()?.get_internal_transactions(option, Some(params)).await?;
    Ok(txs)
}

#[instrument(level = Level::TRACE, err)]
pub async fn get_erc20_token_transfer_events(
    addr: &str,
    contract: &str,
    params: TxListParams,
) -> Result<Vec<ERC20TokenTransferEvent>> {
    let address = addr.parse()?;
    let contract_address = contract.parse()?;
    let option = TokenQueryOption::ByAddressAndContract(address, contract_address);
    // FIXME: This API return Maximum 10,000 records
    let txs = client()?.get_erc20_token_transfer_events(option, Some(params)).await?;
    Ok(txs)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use alloy::primitives::TxHash;
    use foundry_block_explorers::account::Sort;
    use insta::{assert_debug_snapshot, assert_yaml_snapshot};
    use nill::{Nil, nil};

    use super::*;

    const ADDRESS: &str = "0xcf4f5cbc40ab3c8d8b0bfe752f70bf0916c0d938";

    #[tokio::test]
    async fn test_get_transactions() -> Result<Nil> {
        let params = TxListParams {
            start_block: 55911167,
            end_block: 56045281,
            page: 0,
            offset: 1000,
            sort: Sort::Asc,
        };
        let txs = get_transactions(ADDRESS, params).await?;

        let tx_hash = txs[0].hash.value().unwrap();
        let hash = "0xf04e017634abb40ed16e18b19459c4a968cdb6a56dd9888c63b15530cd57e91d";

        assert_eq!(tx_hash, &TxHash::from_str(hash)?);
        assert_eq!(txs.len(), 52);
        Ok(nil)
    }

    #[tokio::test]
    async fn test_get_internal_transactions() -> Result<Nil> {
        let params = TxListParams {
            start_block: 55511167,
            end_block: 56045281,
            page: 0,
            offset: 1000,
            sort: Sort::Asc,
        };
        let txs = get_internal_transactions(ADDRESS, params).await?;

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

    #[tokio::test]
    async fn test_get_erc20_token_transfer_events() -> Result<Nil> {
        const ULTI_TOKEN: &str = "0x0e7779e698052f8fe56c415c3818fcf89de9ac6d";
        let params = TxListParams {
            start_block: 55911167,
            end_block: 56045281,
            page: 0,
            offset: 1000,
            sort: Sort::Asc,
        };
        let txs = get_erc20_token_transfer_events(ADDRESS, ULTI_TOKEN, params).await?;

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
