use crate::fetch::{Fetch, etherscan::client::Etherscan};

pub mod block;
pub mod client;
pub mod contract;
pub mod model;
pub mod transaction;

pub trait EtherscanFetch {
    type Ret;

    type Err;

    fn fetch(self) -> impl Future<Output = Result<Self::Ret, Self::Err>>;
}

impl<T> EtherscanFetch for T
where
    Etherscan: Fetch<Self>,
{
    type Err = <Etherscan as Fetch<Self>>::Err;
    type Ret = <Etherscan as Fetch<Self>>::Ret;

    async fn fetch(self) -> Result<Self::Ret, Self::Err> {
        Etherscan::new().fetch(self).await
    }
}
