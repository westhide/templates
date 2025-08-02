use std::collections::HashMap;

use alloy::primitives::{Address, TxHash, utils::format_units};
use axum::{Json, extract::Query};
use foundry_block_explorers::account::{ERC20TokenTransferEvent, GenesisOption, NormalTransaction};
use serde::{Deserialize, Serialize};

use crate::{
    fetch::etherscan::{
        EtherscanFetch,
        error::Error,
        transaction::{
            get_erc20_token_transfer_events::Params as GetTokenTxParams,
            get_transactions::Params as GetNormalTxParams,
        },
    },
    model::result::Data,
    service::alpha::swap::{Swap, get_swap},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlphaTx {
    pub hash: TxHash,
    pub from: Address,
    pub to: Option<Address>,
    pub value: String,
    pub swap: Swap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxData {
    alpha_tx: Vec<AlphaTx>,
    unknown_normal_tx: Vec<NormalTransaction>,
    unknown_token_tx: Vec<ERC20TokenTransferEvent>,
}

async fn get_impl(get_token_event: GetTokenTxParams) -> Result<TxData, Error> {
    let GetTokenTxParams { address, start_block, end_block, pagination, contract } =
        get_token_event.clone();
    let get_normal_tx = GetNormalTxParams { address, start_block, end_block, pagination };
    let normal_tx = get_normal_tx.fetch().await?;

    let mut unknown_normal_tx = Vec::new();
    let mut normal_tx_map = HashMap::with_capacity(normal_tx.len());
    for tx in normal_tx {
        if let GenesisOption::Some(hash) = &tx.hash {
            normal_tx_map.insert(hash.clone(), tx);
        } else {
            unknown_normal_tx.push(tx);
        }
    }

    let token_tx = get_token_event.fetch().await?;

    let mut unknown_token_tx = Vec::new();
    let mut alpha_tx = Vec::with_capacity(token_tx.len());
    for tx in token_tx {
        debug_assert_eq!(tx.token_symbol, "ULTI");
        if let Some(swap) = get_swap(&tx, &contract, &normal_tx_map)? {
            let hash = tx.hash;
            let from = tx.from;
            let to = tx.to;
            let value = format_units(tx.value, tx.token_decimal)?;

            alpha_tx.push(AlphaTx { hash, from, to, value, swap });
        } else {
            unknown_token_tx.push(tx);
        }
    }

    Ok(TxData { alpha_tx, unknown_normal_tx, unknown_token_tx })
}

pub async fn get(Query(params): Query<GetTokenTxParams>) -> Json<Data<TxData>> {
    let data = get_impl(params).await;
    Json(data.into())
}
