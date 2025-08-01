pub mod etherscan;

// FIXME: use #![feature(inherent_associated_types)]
pub trait Param {
    type Ret;

    type Err;
}

pub trait Fetch<T> {
    type Ret;

    type Err;

    fn fetch(&mut self, params: T) -> impl Future<Output = Result<Self::Ret, Self::Err>>;
}
