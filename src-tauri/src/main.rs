#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod lib;
use lib::calendar::{actions::create_event,utils};

// #[derive(Debug ,serde::Serialize, serde::Deserialize)]
// struct EventFields{
//     name: String,
//     phone: String,
//     email: String,
// }

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// fn create_event(object: EventFields ) -> EventFields {
//     format!("{:?}", object);
//     object
// }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, create_event])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
