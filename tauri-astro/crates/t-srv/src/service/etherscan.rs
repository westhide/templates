use axum::{Json, extract::Query};

use crate::{
    fetch::{
        Param,
        etherscan::{
            EtherscanFetch, block::get_by_timestamp::Params as GetBlockNumberParams,
            transaction::get_transactions::Params as GetTransationsParams,
        },
    },
    model::result::Data,
};

pub async fn get_block_number(
    Query(params): Query<GetBlockNumberParams>,
) -> Json<Data<<GetBlockNumberParams as Param>::Ret>> {
    let data = params.fetch().await;
    Json(data.into())
}

pub async fn get_transactions(
    Query(params): Query<GetTransationsParams>,
) -> Json<Data<<GetTransationsParams as Param>::Ret>> {
    let data = params.fetch().await;
    Json(data.into())
}
