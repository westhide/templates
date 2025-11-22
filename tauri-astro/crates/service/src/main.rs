// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use nill::Nil;
use service_lib::error::Result;

fn main() -> Result<Nil> {
    service_lib::run()
}
