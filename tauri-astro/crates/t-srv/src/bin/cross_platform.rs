// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use nill::{Nil, nil};
use t_lib::log::{error, info, init_tracing_subscriber_log};
use t_srv::trpc;
use tauri::{Result, async_runtime::spawn};

pub mod log {
    use tauri::{Runtime, plugin::TauriPlugin};

    pub fn init_tauri_log_plugin<R: Runtime>() -> TauriPlugin<R> {
        // use tauri_plugin_log::{Builder, Target, TargetKind::Stdout};
        // Builder::new().target(Target::new(Stdout)).build()
        tauri_plugin_log::Builder::new().build()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn main() -> Result<Nil> {
    init_tracing_subscriber_log();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|_| {
            info!("tauri setup");
            spawn(async {
                if let Err(err) = trpc::run().await {
                    error!("rpc run failed: {err}");
                }
                info!("rpc finish");
            });
            Ok(nil)
        })
        .run(tauri::generate_context!())?;

    Ok(nil)
}
