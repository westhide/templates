use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use alloy_chains::Chain;
use foundry_block_explorers::Client;
use rand::random_range;

// https://api.etherscan.io/v2/api?chainid=56&module=contract&action=getabi&address=0x0e7779e698052f8fe56c415c3818fcf89de9ac6d&apikey=K4SDMH5SKWHRNK6G3PIXH1UTCWS1RND9DN

const API_KEY: &[&str] = &[
    // default
    "K4SDMH5SKWHRNK6G3PIXH1UTCWS1RND9DN",
    // txlist
    "1BD7WRF9C2TPVB38TCVY9U1R6F2BVFRM8D",
    "6NTVDK2AAP433UNVGAGD3X4YR25GRXW1I4",
    "M2193Y7ZXYRSJYGZEDC8N2MA54JJUB87ZA",
    "DHIIQTBNFU7JTDZNFHSJJKRIHI87Y41ZNG",
    "PAM5T2NCRJESWY9TV96AUF29YBPNE895IE",
    "AYB7PI37YKAKU84C9B4AIKY2PWRVUNSZG3",
    "QQ4TC8B3Z3F91MJWR95N1YTPZVJMZWQQ3C",
    "6B7PWZP4Z4SDN61B58VW1GHBVATHRFPH4H",
    "U1UMTGEDSQSHCH1M2JYPCVWKT1VT5XA7TK",
    "3ZCQ31SJJIJHTWM98QZW5KK18KKC4E7UZM",
    "82YCUG2S2J1FCM1QERU8PIEKA9CGCH9VBE",
    "X3YGWTZ6IUVUTDEZ13IDZBK98G7DDIV76H",
    // txlistinternal
    "WU6BJG8GEZFRQXHHS4NWAGUXFQ1NV5VQKY",
    "1S74F22M1R4VZNYAJM6FAE46UDVFTCQIUB",
    "PURC6YBDH9R845NP3YCHUT4CIT1E8NB3XF",
    "TZCZNHGWBHEZ13XYC5JNCT639UBH7PDRKG",
    "VJ96HWYYURF6VYHX1EI53U6PNZQNTHRISJ",
    "4DGG9Z1C1J2EGRQCSDUCENM7SKDUGDAF6N",
    "TNVGQD5H1B46VKGYVXU7823P3V3GNA2N37",
    "FQIUSS616PPA3BXIGV5VM3YYKCKSRYXVRS",
    "3I4NKA8YTPQCDV2NU9D76EMUU67CNNVQE7",
    "FS2251G4TVG54DGF5HZ4A4X33E9IITAW9V",
    "6AUFM7CJIS1RPCRWVUIRP59XGJJHPTUXMV",
    "KATCVPX9JZJ6A9RSV7D5SHC5AZEYC4ZH3R",
    // tokentx
    "Y5X8M19FJT8F1Q74A64C11WHGHCFXBY827",
    "PQ7WTNE1G94KAQHPG7TBNUQFQCK451IGCV",
    "Z175JK283IND6VV785RSF33HEES6E9PG3Y",
    "AUD8N76NDUMH4GG2J46CB8RVU9FZ8465E6",
    "UZ2BWTSK9D385PYINEN36D77WV96DX9639",
    "H859Z4W5FFG8MQRE6MHV6QRYCPIUUQ61GP",
    "C99ISPCTBGY4VKAJQ5ZHAQ8IPFWBYVHRIG",
    "A94RSZYSCW9QCS79BNZH1S8DA9KM2ENAEJ",
    "TREFDHHPJY7UFPSTZXSI5PX9TE3VB5S1X7",
    "T4S323ETPFFTT4WGKM3NNV4Z7P72I1YX1H",
    "RW3T58UUIIE3TE66ZRXH4EATKN2GFZN7VB",
    "7TN7C9WME1783WZZYAJSJFRHCR9ZBVUD45",
];

pub struct EtherscanClient {
    client: Client,
}

impl EtherscanClient {
    pub fn init() -> Client {
        let idx = random_range(..API_KEY.len());
        let key = API_KEY[idx];
        let bsc = Chain::bsc_mainnet();
        let timeout = Duration::from_secs(30);

        let http = reqwest::ClientBuilder::new()
            .connect_timeout(timeout)
            .timeout(timeout)
            .build()
            .expect("Http Client");

        Client::builder()
            .with_api_key(key)
            .chain(bsc)
            .expect("BSC Chain")
            .with_client(http)
            .build()
            .expect("Etherscan Client")
    }

    pub fn new() -> Self {
        Self { client: Self::init() }
    }
}

impl Deref for EtherscanClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl DerefMut for EtherscanClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client
    }
}
