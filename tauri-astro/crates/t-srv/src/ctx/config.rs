use std::env;

use config::File;
use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub url: String,
    pub key: Option<Vec<String>>,
}

impl Config {
    pub fn try_new() -> Result<Self> {
        let path = env::var("CONFIG_FILE")?;

        let source = File::with_name(&path).required(true);
        let config_file = config::Config::builder().add_source(source).build()?;

        Ok(config_file.try_deserialize()?)
    }
}
