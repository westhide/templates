use std::collections::HashMap;

use alloy::{
    hex,
    primitives::{Address, U256, utils::format_units},
    sol_types::SolCall,
};
use axum::extract::Query;
use chrono::{Local, NaiveDate, Utc};
use foundry_block_explorers::account::{ERC20TokenTransferEvent, GenesisOption, NormalTransaction};
use serde::{Deserialize, Serialize};
use t_lib::log::{Level, instrument, trace};
use tokio::task::JoinSet;

use crate::{
    error::{Error, Result, err},
    fetch::etherscan::{
        EtherscanFetch,
        block::get_number::Params as GetBlockNumber,
        model::pagination::Pagination,
        transaction::{
            get_erc20_token_transfer_events::Params as GetTokenTx,
            get_transactions::Params as GetNormalTx,
        },
    },
    model::result::ResultData,
    share::{
        contract::proxy_swap_v2::ProxySwapV2,
        datetime::{current_date, current_time, get_date_time, parse_date},
    },
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
pub struct Units {
    value: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

    pub fn fromat_uint(&self, uint: U256) -> Result<Units> {
        let value = format_units(uint, self.decimal())?;
        Ok(Units { value })
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
    pub from_token: Token,
    pub from_value: U256,
    pub from_units: Units,
    pub into: U256,
    pub into_token: Token,
    pub into_value: U256,
    pub into_units: Units,
    pub usdt_value: U256,
    pub usdt_units: Units,
}

impl TryFrom<ProxySwapV2> for Swap {
    type Error = Error;

    fn try_from(call: ProxySwapV2) -> Result<Self, Self::Error> {
        let ProxySwapV2 { from, from_value, into, into_value, .. } = call;

        let from_token = from.try_into()?;
        let into_token = into.try_into()?;

        let usdt_value = match (from_token, into_token) {
            (Token::Usdt, _) => from_value,
            (_, Token::Usdt) => into_value,
            (_, _) => return err!("USDT Token Not Found"),
        };

        let swap = Self {
            from,
            from_units: from_token.fromat_uint(from_value)?,
            from_token,
            from_value,
            into,
            into_units: into_token.fromat_uint(into_value)?,
            into_token,
            into_value,
            usdt_value,
            usdt_units: Token::Usdt.fromat_uint(usdt_value)?,
        };
        Ok(swap)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalTx {
    pub tx: NormalTransaction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap: Option<Swap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

macro_rules! invalid_tx {
    ($tx:expr, $($arg:tt)*) => {
        NormalTx {
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    repeat_token_tx: Vec<ERC20TokenTransferEvent>,
    total: Units,
}

async fn get_data(params: GetTokenTx) -> Result<Data> {
    let GetTokenTx { address, contract, start_block, end_block, pagination } = params.clone();

    let get_normal_tx = GetNormalTx { address, start_block, end_block, pagination };

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
                                    Ok(swap) => NormalTx { swap: Some(swap), cause: None, tx },
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

    let total = normal_tx_list.iter().fold(U256::ZERO, |mut acc, tx| {
        if let Some(swap) = &tx.swap {
            acc += swap.usdt_value;
        }
        acc
    });

    let data = Data {
        normal_tx_list,
        token_tx_list: token_tx_map.into_values().collect(),
        repeat_token_tx,
        total: Token::Usdt.fromat_uint(total)?,
    };
    Ok(data)
}

#[instrument(level = Level::TRACE, skip_all, err)]
pub async fn get_tx(Query(params): Query<GetTokenTx>) -> ResultData<Data> {
    let data = get_data(params).await?;
    Ok(data.into())
}

#[instrument(level = Level::TRACE, skip_all, err)]
pub async fn get_tx_by_date_range(
    Query(params): Query<GetTokenTxByDate>,
) -> ResultData<HashMap<String, Data>> {
    let ranges = params.get_block_ranges().await?;

    let GetTokenTxByDate { address, contract, pagination, .. } = params;

    let mut tx_map = HashMap::new();
    for (date, start_block, end_block) in ranges {
        let get_token_tx = GetTokenTx { address, contract, start_block, end_block, pagination };
        let data = get_data(get_token_tx).await?;
        tx_map.insert(date, data);
    }
    Ok(tx_map.into())
}

#[instrument(level = Level::TRACE, skip_all, err)]
pub async fn get_total_usdt_unit(Query(params): Query<GetTokenTx>) -> ResultData<String> {
    let data = get_data(params).await?;
    Ok(data.total.value.into())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTokenTxByDate {
    pub address: Address,
    pub contract: Address,
    pub start_date: String,
    pub end_date: String,
    #[serde(default, flatten)]
    pub pagination: Option<Pagination>,
}

impl GetTokenTxByDate {
    pub async fn get_block_range(date: &NaiveDate) -> Result<(String, u64, u64)> {
        let from_time = get_date_time(date, 0, 0, 0)?;
        let mut into_time = get_date_time(date, 23, 59, 59)?;

        let now = current_time().timestamp() as u64;
        if into_time > now {
            into_time = now
        }

        let from =
            GetBlockNumber { timestamp: from_time, closest: "before".into() }.fetch().await?;
        let into = GetBlockNumber { timestamp: into_time, closest: "after".into() }.fetch().await?;
        let date_fmt = format!("{}", date.format("%Y-%m-%d"));
        Ok((date_fmt, from, into))
    }

    pub async fn get_block_ranges(&self) -> Result<Vec<(String, u64, u64)>> {
        let from = parse_date(&self.start_date)?;
        let into = parse_date(&self.end_date)?;

        let current_date = current_date();
        if into > current_date {
            return err!("Invalid end_date: {into}, Current: {current_date:?}");
        }

        if from > into {
            return err!("Invalid Date Range")
        }

        let mut date = from;
        let mut ranges = Vec::new();
        let mut handler: JoinSet<Result<(String, u64, u64)>> = JoinSet::new();
        while date <= into {
            handler.spawn(async move {
                trace!("Get date: {date:?} block range");
                Self::get_block_range(&date).await
            });
            date = match date.succ_opt() {
                Some(date) => date,
                None => return err!("Last Representable Date"),
            };
        }
        while let Some(result) = handler.join_next().await {
            let (date, from, into) = result??;
            ranges.push((date, from, into));
        }
        Ok(ranges)
    }
}

#[instrument(level = Level::TRACE, skip_all, err)]
pub async fn get_total_usdt_unit_by_date_range(
    Query(params): Query<GetTokenTxByDate>,
) -> ResultData<HashMap<String, String>> {
    let ranges = params.get_block_ranges().await?;

    let GetTokenTxByDate { address, contract, pagination, .. } = params;

    let mut usdt_units = HashMap::new();
    let mut handler: JoinSet<Result<(String, String)>> = JoinSet::new();
    for (date, start_block, end_block) in ranges {
        handler.spawn(async move {
            let get_token_tx = GetTokenTx { address, contract, start_block, end_block, pagination };

            trace!("Get date: {date:?} transaction");
            let data = get_data(get_token_tx).await?;
            Ok((date, data.total.value))
        });
    }

    while let Some(result) = handler.join_next().await {
        let (date, value) = result??;
        usdt_units.insert(date, value);
    }

    Ok(usdt_units.into())
}
