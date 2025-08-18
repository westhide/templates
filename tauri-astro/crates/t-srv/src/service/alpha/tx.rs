use std::collections::HashMap;

use alloy::{
    hex,
    primitives::{Address, U256, utils::format_units},
    sol_types::SolCall,
};
use axum::extract::Query;
use chrono::NaiveDate;
use foundry_block_explorers::account::{ERC20TokenTransferEvent, GenesisOption, NormalTransaction};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use t_lib::log::{Level, instrument, trace};
use tokio::task::JoinSet;

use crate::{
    error::{Error, Result, err},
    fetch::etherscan::{
        EtherscanFetch,
        block::get_number::{Closest, Params as GetBlockNumber},
        model::pagination::Pagination,
        transaction::{
            get_erc20_token_transfer_events::Params as GetTokenTx,
            get_transactions::Params as GetNormalTx,
        },
    },
    model::result::ResultData,
    share::{
        contract::ProxySwapV2,
        datetime::{current_date, current_time, get_date_time, parse_date},
    },
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Flag {
    From,
    Into,
}

impl Flag {
    #[rustfmt::skip]
    pub fn try_get(tx: &NormalTransaction, addr: &Address) -> Option<Self> {
        if let GenesisOption::Some(hash) = &tx.from && hash == addr {
            return Some(Flag::From);
        }
        if let Some(hash) = &tx.to && hash == addr {
            return Some(Flag::Into);
        }
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Units {
    pub value: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Token {
    Usdt,
    Ulti,
}

impl Token {
    // ULTI
    pub const ULTI_ADDRESS: Address = Address::new(Token::ULTI_BYTES);
    pub const ULTI_BYTES: [u8; 20] = hex!("0x0e7779e698052f8fe56c415c3818fcf89de9ac6d");
    pub const ULTI_DECIMAL: u8 = 18;
    pub const ULTI_TOKEN: U256 = U256::from_be_slice(&Token::ULTI_BYTES);
    // USDT
    pub const USDT_ADDRESS: Address = Address::new(Token::USDT_BYTES);
    pub const USDT_BYTES: [u8; 20] = hex!("0x55d398326f99059ff775485246999027b3197955");
    pub const USDT_DECIMAL: u8 = 18;
    pub const USDT_TOKEN: U256 = U256::from_be_slice(&Token::USDT_BYTES);

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

    fn try_from(token: U256) -> Result<Self, Self::Error> {
        match token {
            Self::ULTI_TOKEN => Ok(Self::Ulti),
            Self::USDT_TOKEN => Ok(Self::Usdt),
            token => err!("Unknown Token: 0x{token:x}"),
        }
    }
}

impl TryFrom<Address> for Token {
    type Error = Error;

    fn try_from(addr: Address) -> Result<Self, Self::Error> {
        match addr {
            Self::ULTI_ADDRESS => Ok(Self::Ulti),
            Self::USDT_ADDRESS => Ok(Self::Usdt),
            addr => err!("Unknown Token Address: 0x{addr:x}"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Swap {
    pub flag: Flag,
    pub from_token: Token,
    pub from_token_with_fee: U256,
    pub from_value: U256,
    pub from_units: Units,
    pub into_token: Token,
    pub into_token_with_fee: U256,
    pub into_value: U256,
    pub into_units: Units,
}

impl Swap {
    #[inline]
    pub fn usdt_value(&self) -> U256 {
        match self.flag {
            Flag::From => self.from_value,
            Flag::Into => U256::ZERO,
        }
    }
}

impl TryFrom<ProxySwapV2> for Swap {
    type Error = Error;

    fn try_from(call: ProxySwapV2) -> Result<Self, Self::Error> {
        let ProxySwapV2 { fromTokenWithFee, fromAmt, intoTokenWithFee, minReturnAmt, .. } = call;

        let from_token = fromTokenWithFee.try_into()?;
        let into_token = intoTokenWithFee.try_into()?;

        let flag = match (from_token, into_token) {
            (Token::Usdt, _) => Flag::From,
            (_, Token::Usdt) => Flag::Into,
            (_, _) => return err!("USDT Token Not Found"),
        };

        let swap = Self {
            flag,
            from_token,
            from_token_with_fee: fromTokenWithFee,
            from_units: from_token.fromat_uint(fromAmt)?,
            from_value: fromAmt,
            into_token,
            into_token_with_fee: intoTokenWithFee,
            into_units: into_token.fromat_uint(minReturnAmt)?,
            into_value: minReturnAmt,
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
    total_usdt: Units,
}

async fn get_data(params: GetTokenTx) -> Result<Data> {
    let GetTokenTx { address, contract, start_block, end_block, pagination } = params.clone();

    let token = Token::try_from(contract)?;

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
        let Some(_flag) = Flag::try_get(&tx, &address) else {
            normal_tx_list.push(invalid_tx!(tx, "Unknown Tx Trade"));
            continue;
        };
        if tx.is_error != "0" || tx.tx_receipt_status != "1" {
            normal_tx_list.push(invalid_tx!(tx, "Tx Trade Fail"));
            continue;
        }

        let normal_tx = match &tx.hash {
            GenesisOption::Some(hash) => {
                match &tx.method_id {
                    // TODO: Swap method_id
                    Some(method_id) if method_id == &ProxySwapV2::METHOD_ID => {
                        match token_tx_map.get(hash) {
                            Some(_token_tx) => {
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

    let total_usdt = normal_tx_list.iter().fold(U256::ZERO, |mut acc, tx| {
        if let Some(swap) = &tx.swap {
            acc += swap.usdt_value();
        }
        acc
    });

    let data = Data {
        normal_tx_list,
        token_tx_list: token_tx_map.into_values().collect(),
        repeat_token_tx,
        total_usdt: Token::Usdt.fromat_uint(total_usdt + total_usdt)?,
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
) -> ResultData<IndexMap<String, Data>> {
    let ranges = params.get_block_ranges().await?;

    let GetTokenTxByDate { address, contract, pagination, .. } = params;

    let mut tx_map = IndexMap::new();
    for (date, start_block, end_block) in ranges {
        let get_token_tx = GetTokenTx { address, contract, start_block, end_block, pagination };
        let data = get_data(get_token_tx).await?;
        tx_map.insert(date, data);
    }
    Ok(tx_map.into())
}

#[instrument(level = Level::TRACE, skip_all, err)]
pub async fn get_total_usdt_unit(Query(params): Query<GetTokenTx>) -> ResultData<Units> {
    let data = get_data(params).await?;
    Ok(data.total_usdt.into())
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

        let from = GetBlockNumber::new(from_time, Closest::Next).fetch().await?;
        let into = GetBlockNumber::new(into_time, Closest::Prev).fetch().await?;
        let date_fmt = format!("{}", date.format("%Y-%m-%d"));
        trace!("{date_fmt} block: {from}..{into}");
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
) -> ResultData<IndexMap<String, Units>> {
    let ranges = params.get_block_ranges().await?;

    let GetTokenTxByDate { address, contract, pagination, .. } = params;

    let mut usdt_units = IndexMap::new();
    let mut handler: JoinSet<Result<(String, Units)>> = JoinSet::new();
    for (date, start_block, end_block) in ranges {
        handler.spawn(async move {
            let get_token_tx = GetTokenTx { address, contract, start_block, end_block, pagination };
            trace!("Get date: {date:?} transaction");
            let data = get_data(get_token_tx).await?;
            Ok((date, data.total_usdt))
        });
    }

    while let Some(result) = handler.join_next().await {
        let (date, value) = result??;
        usdt_units.insert(date, value);
    }

    Ok(usdt_units.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn batch_query() {
        let addrs = vec![
            "0x6b29f3C0360e68e41d828D347B0390eB10F26f20",
            "0xA7bfa85c6243d5DcEAE1A8465917f2CB3355BD4d",
            "0xc63bCcfc9DAE113F137f3137cB91FCA709269970",
            "0xdB2683192ed191009903D627b9407D551706dCE2",
            "0x6FF65A8E18FC5E4DeD66F6eECFb67bfD299b2F3B",
            "0x9221ecE8Eaa6a7bA05A2f424FCDDB731Ff44c9f9",
            "0x9009e7B4cA28825E1401F221BEBd365aAa3a6705",
            "0x38a448f4f03cf1b8b263e3004a3dfd6cfeeaeabb",
            "0xe56b5D70CF77D12166d22b4402Bf652Af1AFAd7D",
            "0x8a9227D31255635cF992FC3760dED9822Dc990eB",
            "0xac1d9644b03901db9b9de2b3ea26226c6384db59",
            "0x738a4800Fb161A93f8A234CDd11C5f42B754b759",
            "0x5cC82990B1Fff2A88368762D6F09e9394b9983d1",
            "0x77c4511b5c0F58B62D306Df80816e1BDf33f0939",
            "0x87bBB74511570cfb9BfCd02F1acfEF024d398Fc4",
        ];

        for addr in addrs {
            println!("query: {addr}");
            let ret = get_total_usdt_unit_by_date_range(Query(GetTokenTxByDate {
                address: addr.parse().unwrap(),
                contract: "0x0e7779e698052f8fe56c415c3818fcf89de9ac6d".parse().unwrap(),
                start_date: "2025-08-10".into(),
                end_date: "2025-08-15".into(),
                pagination: None,
            }))
            .await;
            let msg = match ret {
                Ok(r) => serde_json::to_string(&r).unwrap(),
                Err(err) => format!("Failed: {addr}, {err}"),
            };
            println!("{msg:?}");
        }
    }
}
