use alloy::{primitives::Bytes, sol};

// proxySwapV2(address router,uint256 fromTokenWithFee,uint256 fromAmt,uint256 toTokenWithFee,uint256 minReturnAmt,bytes callData)
sol! {
    function proxySwapV2(
        address router,
        uint256 from,
        uint256 from_value,
        uint256 into,
        uint256 into_value,
        bytes call_data
    );
}

pub type ProxySwapV2 = proxySwapV2Call;

impl ProxySwapV2 {
    pub const METHOD_ID: Bytes = Bytes::from_static(Self::METHOD_ID_BYTES);
    pub const METHOD_ID_BYTES: &[u8] = &[0xE5, 0xE8, 0x89, 0x4B];
}
