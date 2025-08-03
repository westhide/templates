use std::future;

use axum::http::{Request, Response};
use t_lib::log::warn;
use tower::retry::{Policy, RetryLayer};

#[derive(Debug, Clone)]
pub struct Retry {
    num: usize,
}

impl Retry {
    pub fn new(num: usize) -> Self {
        Self { num }
    }
}

impl<T, U, E> Policy<Request<T>, Response<U>, E> for Retry
where
    T: Clone,
{
    type Future = future::Ready<()>;

    fn retry(
        &mut self,
        _: &mut Request<T>,
        result: &mut Result<Response<U>, E>,
    ) -> Option<Self::Future> {
        if result.is_err() && self.num > 0 {
            warn!("request retry...");
            self.num -= 1;
            Some(future::ready(()))
        } else {
            None
        }
    }

    fn clone_request(&mut self, req: &Request<T>) -> Option<Request<T>> {
        Some(req.clone())
    }
}

pub fn retry(num: usize) -> RetryLayer<Retry> {
    let retry = Retry::new(num);
    RetryLayer::new(retry)
}
