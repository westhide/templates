use std::collections::HashMap;

use alloy::primitives::{Address, TxHash, utils::format_units};
use foundry_block_explorers::account::{ERC20TokenTransferEvent, GenesisOption, NormalTransaction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Symbol {
    #[default]
    Unknown,
    Usdt,
    Usdc,
    Token(String),
}

pub enum Metch {}

const SWAP_ROUTES: &[(&str, &str)] = &[("ULTI", "0xe5e8894b")];

static SWAP_ABI_MAP:  LazyLock<HashMap<>> =

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Swap {
    pub symbol: Symbol,
}

pub fn get_swap(
    tx: &ERC20TokenTransferEvent,
    contract: &Address,
    normal_tx_map: &HashMap<TxHash, NormalTransaction>,
) -> Option<Swap> {
    let normal_tx = normal_tx_map.get(&tx.hash)?;
    None
}
