use alloy::sol;

sol! {
    function proxySwapV2(
        address router,
        uint256 fromTokenWithFee,
        uint256 fromAmt,
        uint256 toTokenWithFee,
        uint256 minReturnAmt,
        bytes callData
    );
}
