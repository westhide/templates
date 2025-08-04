use std::{
    error::Error,
    pin::Pin,
    task::{Context, Poll},
};

use axum::error_handling::HandleErrorLayer;
use nill::Nil;

use crate::model::result::Data;

pub async fn handler<E>(err: E) -> Data<Nil>
where
    E: Error,
{
    Data::fail(err)
}
