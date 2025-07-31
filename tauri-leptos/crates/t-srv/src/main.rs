// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use nill::Nil;
use tauri::Result;

fn main() -> Result<Nil> {
    t_lib::log::init_tracing_subscriber_log();
    t_srv::run()
}
