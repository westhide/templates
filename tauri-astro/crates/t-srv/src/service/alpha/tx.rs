use std::collections::HashMap;

use alloy::{
    hex,
    primitives::{Address, U256, utils::format_units},
    sol_types::SolCall,
};
use axum::extract::Query;
use foundry_block_explorers::account::{ERC20TokenTransferEvent, GenesisOption, NormalTransaction};
use serde::{Deserialize, Serialize};
use t_lib::log::{Level, instrument, trace};

use crate::{
    error::{Error, err},
    fetch::etherscan::{
        EtherscanFetch,
        transaction::{
            get_erc20_token_transfer_events::Params as GetTokenTxParams,
            get_transactions::Params as GetNormalTxParams,
        },
    },
    model::result::ResultData,
    share::contract::proxy_swap_v2::ProxySwapV2,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Trade {
    Send,
    Recv,
}

impl Trade {
    #[rustfmt::skip]
    pub fn try_get(tx: &NormalTransaction, address: &Address) -> Option<Self> {
        if let GenesisOption::Some(hash) = &tx.from && hash == address {
            return Some(Trade::Send);
        }
        if let Some(hash) = &tx.to && hash == address {
            return Some(Trade::Recv);
        }
        None
    }

    pub fn is_tx_fail(tx: &NormalTransaction) -> bool {
        tx.is_error != "0" || tx.tx_receipt_status != "1"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Token {
    Usdt,
    Ulti,
}

impl Token {
    pub const ULTI: U256 = U256::from_be_slice(&hex!("0x0e7779e698052f8fe56c415c3818fcf89de9ac6d"));
    pub const ULTI_DECIMAL: u8 = 18;
    pub const USDT: U256 = U256::from_be_slice(&hex!("0x55d398326f99059ff775485246999027b3197955"));
    pub const USDT_DECIMAL: u8 = 18;

    pub fn decimal(&self) -> u8 {
        match self {
            Token::Usdt => Self::USDT_DECIMAL,
            Token::Ulti => Self::ULTI_DECIMAL,
        }
    }
}

impl TryFrom<U256> for Token {
    type Error = Error;

    fn try_from(contract: U256) -> Result<Self, Self::Error> {
        match contract {
            Self::ULTI => Ok(Self::Ulti),
            Self::USDT => Ok(Self::Usdt),
            contract => err!("Unknown Token Contract: 0x{contract:x}"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Swap {
    pub from: U256,
    pub from_value: U256,
    pub into: U256,
    pub into_value: U256,
    pub usdt: String,
}

impl TryFrom<ProxySwapV2> for Swap {
    type Error = Error;

    fn try_from(call: ProxySwapV2) -> Result<Self, Self::Error> {
        let ProxySwapV2 { from, from_value, into, into_value, .. } = call;

        let usdt_value = match (from, into) {
            (Token::USDT, _) => from_value,
            (_, Token::USDT) => into_value,
            (_, _) => return err!("USDT Token Not Found"),
        };

        let swap = Self {
            from,
            from_value,
            into,
            into_value,
            usdt: format_units(usdt_value, Token::USDT_DECIMAL)?,
        };
        Ok(swap)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalTx {
    pub tx: NormalTransaction,
    pub valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap: Option<Swap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

macro_rules! invalid_tx {
    ($tx:expr, $($arg:tt)*) => {
        NormalTx {
            valid: false,
            swap: None,
            cause: Some(format!($($arg)*)),
            tx: $tx,
        }
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    normal_tx_list: Vec<NormalTx>,
    token_tx_list: Vec<ERC20TokenTransferEvent>,
    repeat_token_tx: Vec<ERC20TokenTransferEvent>,
    total: String,
}

pub async fn get(Query(params): Query<GetTokenTxParams>) -> ResultData<Data> {
    let GetTokenTxParams { address, contract, start_block, end_block, pagination } = params.clone();

    let get_normal_tx = GetNormalTxParams { address, start_block, end_block, pagination };

    let raw_token_tx = params.fetch().await?;

    let mut repeat_token_tx = Vec::new();
    let mut token_tx_map = HashMap::with_capacity(raw_token_tx.len());
    for tx in raw_token_tx {
        if let Some(rep) = token_tx_map.insert(tx.hash.clone(), tx) {
            repeat_token_tx.push(rep);
        }
    }

    let raw_normal_tx = get_normal_tx.fetch().await?;

    let mut normal_tx_list = Vec::with_capacity(raw_normal_tx.len());
    // TODO: filter token
    for tx in raw_normal_tx {
        let Some(ty) = Trade::try_get(&tx, &address) else {
            normal_tx_list.push(invalid_tx!(tx, "Unknown Tx Trade"));
            continue;
        };
        if Trade::is_tx_fail(&tx) {
            normal_tx_list.push(invalid_tx!(tx, "Tx Trade Fail"));
            continue;
        }

        let normal_tx = match &tx.hash {
            GenesisOption::Some(hash) => {
                match &tx.method_id {
                    // TODO: Swap method_id
                    Some(method_id) if method_id == &ProxySwapV2::METHOD_ID => {
                        match token_tx_map.get(hash) {
                            Some(token_tx) => {
                                let call = ProxySwapV2::abi_decode(&tx.input)?;
                                match Swap::try_from(call) {
                                    Ok(swap) => {
                                        NormalTx { valid: true, swap: Some(swap), cause: None, tx }
                                    },
                                    Err(err) => invalid_tx!(tx, "{err}"),
                                }
                            },
                            None => invalid_tx!(tx, "Token Tx Not Found"),
                        }
                    },
                    method_id => invalid_tx!(tx, "Unknown Tx method_id: {method_id:?}"),
                }
            },
            hash => invalid_tx!(tx, "Invalid Tx Hash: {hash:?}"),
        };
        normal_tx_list.push(normal_tx);
    }

    let data = Data {
        normal_tx_list,
        token_tx_list: token_tx_map.into_values().collect(),
        repeat_token_tx,
        total: "".into(),
    };
    Ok(data.into())
}
