use axum::extract::Query;
use serde::Serialize;

pub use crate::fetch::etherscan::{
    block::get_number::Params as GetBlockNumber,
    transaction::{
        get_erc20_token_transfer_events::Params as GetTokenTx,
        get_internal_transactions::Params as GetInternalTx,
        get_transactions::Params as GetNormalTx,
    },
};
use crate::{fetch::etherscan::EtherscanFetch, model::result::Data};

pub type FetchData<T> = Result<Data<<T as EtherscanFetch>::Ret>, <T as EtherscanFetch>::Err>;

pub trait EtherscanGet: EtherscanFetch + Sized {
    fn get(_: Query<Self>) -> impl Future<Output = FetchData<Self>>;
}

impl<T> EtherscanGet for T
where
    T: EtherscanFetch,
    <T as EtherscanFetch>::Ret: Serialize,
{
    async fn get(Query(params): Query<Self>) -> FetchData<Self> {
        let data = params.fetch().await?;
        Ok(data.into())
    }
}
