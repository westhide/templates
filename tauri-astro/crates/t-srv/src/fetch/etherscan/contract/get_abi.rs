use alloy::{json_abi::JsonAbi, primitives::Address};
use serde::{Deserialize, Serialize};
use t_lib::log::{Level, instrument};

use crate::{
    error::Error,
    fetch::{Fetch, Param, etherscan::client::Etherscan},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
    pub contract: Address,
}

impl Param for Params {
    type Err = Error;
    type Ret = JsonAbi;
}

impl Fetch<Params> for Etherscan {
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
    use alloy::primitives::address;
    use insta::assert_debug_snapshot;
    use nill::{Nil, nil};

    use super::*;
    use crate::fetch::etherscan::EtherscanFetch;

    const PROXY_SWAP_V2: Address = address!("0xb300000b72deaeb607a12d5f54773d1c19c7028d");

    #[tokio::test]
    async fn test_get_abi_proxy_swap_v2() -> Result<Nil, Error> {
        let param = Params { contract: PROXY_SWAP_V2 };
        let abi = param.fetch().await?;
        let json = serde_json::to_string(&abi)?;
        assert_debug_snapshot!(json, @r#""[{\"type\":\"constructor\",\"inputs\":[{\"name\":\"_contractOwner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_diamondCutFacet\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"payable\"},{\"type\":\"fallback\",\"stateMutability\":\"payable\"},{\"type\":\"receive\",\"stateMutability\":\"payable\"},{\"type\":\"event\",\"name\":\"DiamondCut\",\"inputs\":[{\"name\":\"_diamondCut\",\"type\":\"tuple[]\",\"indexed\":false,\"internalType\":\"struct IDiamondCut.FacetCut[]\",\"components\":[{\"name\":\"facetAddress\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"action\",\"type\":\"uint8\",\"internalType\":\"enum IDiamondCut.FacetCutAction\"},{\"name\":\"functionSelectors\",\"type\":\"bytes4[]\",\"internalType\":\"bytes4[]\"}]},{\"name\":\"_init\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"address\"},{\"name\":\"_calldata\",\"type\":\"bytes\",\"indexed\":false,\"internalType\":\"bytes\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"error\",\"name\":\"CalldataEmptyButInitNotZero\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"FacetAddressIsNotZero\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"FacetAddressIsZero\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"FacetContainsNoCode\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"FunctionAlreadyExists\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"FunctionDoesNotExist\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"FunctionIsImmutable\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"IncorrectFacetCutAction\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"InitReverted\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"InitZeroButCalldataNotEmpty\",\"inputs\":[]},{\"type\":\"error\",\"name\":\"NoSelectorsInFace\",\"inputs\":[]}]""#);
        Ok(nil)
    }
}
