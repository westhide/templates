use axum::extract::Query;

use crate::{
    fetch::{
        Param,
        etherscan::{
            EtherscanFetch,
            block::get_number::Params as GetBlockNumberParams,
            transaction::{
                get_erc20_token_transfer_events::Params as GetTokenTxParams,
                get_internal_transactions::Params as GetInternalTxParams,
                get_transactions::Params as GetNormalTxParams,
            },
        },
    },
    model::result::ResultData,
};

pub async fn get_block_number(
    Query(params): Query<GetBlockNumberParams>,
) -> ResultData<<GetBlockNumberParams as Param>::Ret> {
    let data = params.fetch().await?;
    Ok(data.into())
}

pub async fn get_normal_tx(
    Query(params): Query<GetNormalTxParams>,
) -> ResultData<<GetNormalTxParams as Param>::Ret> {
    let data = params.fetch().await?;
    Ok(data.into())
}

pub async fn get_internal_tx(
    Query(params): Query<GetInternalTxParams>,
) -> ResultData<<GetInternalTxParams as Param>::Ret> {
    let data = params.fetch().await?;
    Ok(data.into())
}

pub async fn get_token_tx(
    Query(params): Query<GetTokenTxParams>,
) -> ResultData<<GetTokenTxParams as Param>::Ret> {
    let data = params.fetch().await?;
    Ok(data.into())
}
