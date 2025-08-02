use std::collections::HashMap;

use alloy::{
    primitives::{Address, Bytes, TxHash, U256, utils::format_units},
    sol_types::SolCall,
};
use foundry_block_explorers::account::{ERC20TokenTransferEvent, GenesisOption, NormalTransaction};
use serde::{Deserialize, Serialize};
use t_lib::log::{Level, instrument, trace};

use crate::{fetch::etherscan::error::Result, share::contract::proxy_swap_v2::proxySwapV2Call};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Symbol {
    #[default]
    Unknown,
    Usdt,
    Usdc,
    Token(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Swap {
    pub router: Address,
    pub from_token_with_fee: U256,
    pub from_amt: U256,
    pub to_token_with_fee: U256,
    pub min_return_amt: U256,
}

impl From<proxySwapV2Call> for Swap {
    fn from(swap_call: proxySwapV2Call) -> Self {
        let proxySwapV2Call {
            router, fromTokenWithFee, fromAmt, toTokenWithFee, minReturnAmt, ..
        } = swap_call;

        Self {
            router,
            from_token_with_fee: fromTokenWithFee,
            from_amt: fromAmt,
            to_token_with_fee: toTokenWithFee,
            min_return_amt: minReturnAmt,
        }
    }
}

pub fn get_swap(
    tx: &ERC20TokenTransferEvent,
    contract: &Address,
    normal_tx_map: &HashMap<TxHash, NormalTransaction>,
) -> Result<Option<Swap>> {
    match normal_tx_map.get(&tx.hash) {
        Some(normal_tx) => {
            // TODO: Swap method_id
            let swap_call = proxySwapV2Call::abi_decode(&normal_tx.input)?;
            Ok(Some(swap_call.into()))
        },
        None => Ok(None),
    }
}
