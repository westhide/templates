use foundry_block_explorers::blocks::BlockNumberByTimestamp;
use t_lib::log::{Level, instrument};

use crate::api::etherscan::{client::client, error::Result};

#[instrument(level = Level::TRACE, err)]
pub async fn get_block_by_timestamp(timestamp: u64) -> Result<BlockNumberByTimestamp> {
    let ret = client()?.get_block_by_timestamp(timestamp, "before").await?;
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use alloy::primitives::U64;
    use nill::{Nil, nil};

    use super::*;

    #[tokio::test]
    async fn test_get_block_by_timestamp() -> Result<Nil> {
        let ret = get_block_by_timestamp(1754024487).await?;
        assert_eq!(ret.block_number, U64::from(56035281).into());
        Ok(nil)
    }
}
