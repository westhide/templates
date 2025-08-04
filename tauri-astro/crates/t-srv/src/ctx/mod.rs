use crate::{ctx::config::Config, error::Result};

pub mod config;

#[derive(Debug)]
pub struct Context {
    pub config: Config,
}

impl Context {
    pub fn try_new() -> Result<Self> {
        let config = Config::try_new()?;
        Ok(Self { config })
    }
}
