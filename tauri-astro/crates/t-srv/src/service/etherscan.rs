use axum::{Json, extract::Query};

use crate::{
    fetch::{
        Param,
        etherscan::{
            EtherscanFetch,
            block::get_by_timestamp::Params as GetBlockNumberParams,
            transaction::{
                get_erc20_token_transfer_events::Params as GetTokenTxParams,
                get_internal_transactions::Params as GetCexTxParams,
                get_transactions::Params as GetNormalTxParams,
            },
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

pub async fn get_normal_tx(
    Query(params): Query<GetNormalTxParams>,
) -> Json<Data<<GetNormalTxParams as Param>::Ret>> {
    let data = params.fetch().await;
    Json(data.into())
}

pub async fn get_internal_tx(
    Query(params): Query<GetCexTxParams>,
) -> Json<Data<<GetCexTxParams as Param>::Ret>> {
    let data = params.fetch().await;
    Json(data.into())
}

pub async fn get_token_tx(
    Query(params): Query<GetTokenTxParams>,
) -> Json<Data<<GetTokenTxParams as Param>::Ret>> {
    let data = params.fetch().await;
    Json(data.into())
}
