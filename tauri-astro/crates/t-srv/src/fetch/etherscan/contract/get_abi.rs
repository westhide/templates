use alloy::{json_abi::JsonAbi, primitives::Address};
use serde::{Deserialize, Serialize};
use t_lib::log::{Level, instrument};

use crate::fetch::{
    Fetch, Param,
    etherscan::{client::EtherscanClient, error::Error},
};

pub const ABI_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/fetch/etherscan/contract/abi");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
    pub contract: Address,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub block_number: usize,
}

impl Param for Params {
    type Err = Error;
    type Ret = JsonAbi;
}

impl Fetch<Params> for EtherscanClient {
    type Err = <Params as Param>::Err;
    type Ret = <Params as Param>::Ret;

    #[instrument(level = Level::TRACE, skip_all, err, fields(?params))]
    async fn fetch(&mut self, params: Params) -> Result<Self::Ret, Self::Err> {
        let Params { contract } = params;
        let abi = self.contract_abi(contract).await?;
        Ok(abi)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use insta::assert_debug_snapshot;
    use nill::{Nil, nil};

    use super::*;
    use crate::fetch::etherscan::{EtherscanFetch, error::Result};

    const ULTI_TOKEN: &str = "0x0e7779e698052f8fe56c415c3818fcf89de9ac6d";

    fn save_abi(name: &str, abi: &JsonAbi) -> Result<Nil> {
        let path = format!("{ABI_DIR}/{name}.json");
        println!("{path}");
        let json = serde_json::to_string(&abi)?;
        fs::write(path, json)?;
        Ok(nil)
    }

    #[tokio::test]
    async fn test_get_abi_ulti() -> Result<Nil> {
        let param = Params { contract: ULTI_TOKEN.parse()? };
        let abi = param.fetch().await?;

        assert_debug_snapshot!(&abi);
        save_abi("ULTI", &abi)?;
        Ok(nil)
    }
}
