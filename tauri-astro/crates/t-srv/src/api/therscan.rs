use reqwest::Url;
use serde::{Deserialize, Serialize};
use t_lib::log::{Level, instrument};

use crate::api::Error;

const BASE_URL: &str =
    "https://api.etherscan.io/v2/api?apikey=K4SDMH5SKWHRNK6G3PIXH1UTCWS1RND9DN&chainid=56";

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBlockNoByTimeRet {
    pub status: String,
    pub message: String,
    pub result: String,
}

#[instrument(level = Level::TRACE, err)]
pub async fn getblocknobytime(timestamp: i64) -> Result<GetBlockNoByTimeRet, Error> {
    let mut url = Url::parse_with_params(BASE_URL, &[
        ("module", "block"),
        ("action", "getblocknobytime"),
        ("closest", "before"),
    ])?;
    let timestamp_pair = ("timestamp", format!("{timestamp}"));
    url.query_pairs_mut().extend_pairs(Some(timestamp_pair));
    let ret = reqwest::get(url).await?.json().await?;
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use nill::{Nil, nil};

    use super::*;

    #[tokio::test]
    async fn test_getblocknobytime() -> Result<Nil, Error> {
        let now = Utc::now();
        let timestamp = now.timestamp();
        let ret = getblocknobytime(timestamp).await?;

        println!("{ret:?}");

        Ok(nil)
    }
}
