pub mod log;
pub mod rpc;

use nill::{Nil, nil};
use t_lib::log::{error, info, trace};
use tauri::{Result, async_runtime::spawn};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    trace!("Greet");
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<Nil> {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|_| {
            info!("tauri setup");
            spawn(async {
                if let Err(err) = rpc::run().await {
                    error!("rpc run failed: {err}");
                }
                info!("rpc finish");
            });
            Ok(nil)
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
}
