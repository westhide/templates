const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn get() -> String {
    PKG_VERSION.into()
}
