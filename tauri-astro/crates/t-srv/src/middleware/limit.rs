use std::time::Duration;

use tower::{
    limit::{ConcurrencyLimitLayer, RateLimitLayer},
    timeout::TimeoutLayer,
};

pub fn rate(num: u64, sec: u64) -> RateLimitLayer {
    RateLimitLayer::new(num, Duration::from_secs(sec))
}

pub fn concurrent(max: usize) -> ConcurrencyLimitLayer {
    ConcurrencyLimitLayer::new(max)
}

pub fn timeout(sec: u64) -> TimeoutLayer {
    TimeoutLayer::new(Duration::from_secs(sec))
}
