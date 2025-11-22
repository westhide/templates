pub mod error;

use std::error::Error as StdError;

use nill::{Nil, nil};
use tauri::{App, Manager, generate_context, generate_handler};
use tauri_plugin_opener::init as opener;
use lib::log;

use crate::error::Result;

#[tauri::command]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(debug_assertions)]
fn open_devtools(app: &mut App) {
    if let Some(window) = app.get_webview_window("main") {
        log::info!("open devtools");
        window.open_devtools();
    } else {
        log::warn!("skip devtools");
    }
}

fn setup(app: &mut App) -> Result<Nil, Box<dyn StdError>> {
    // #[cfg(debug_assertions)]
    // open_devtools(app);

    Ok(nil)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<Nil> {
    log::init_tracing_subscriber_log();

    tauri::Builder::default()
        .plugin(opener())
        .setup(setup)
        .invoke_handler(generate_handler![version])
        .run(generate_context!())?;

    Ok(nil)
}
