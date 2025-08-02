use crate::fetch::{Fetch, etherscan::client::EtherscanClient};

pub mod block;
pub mod client;
pub mod contract;
pub mod error;
pub mod model;
pub mod transaction;

pub trait EtherscanFetch {
    type Ret;

    type Err;

    fn fetch(self) -> impl Future<Output = Result<Self::Ret, Self::Err>>;
}

impl<T> EtherscanFetch for T
where
    EtherscanClient: Fetch<Self>,
{
    type Err = <EtherscanClient as Fetch<Self>>::Err;
    type Ret = <EtherscanClient as Fetch<Self>>::Ret;

    async fn fetch(self) -> Result<Self::Ret, Self::Err> {
        EtherscanClient::new().fetch(self).await
    }
}
