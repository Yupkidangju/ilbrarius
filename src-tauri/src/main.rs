pub mod crawler;
pub mod printer;
pub mod store;

use tauri::Manager;
use crate::crawler::normalize_url;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn track_url(url: String) -> Result<String, String> {
    match normalize_url(&url).await {
        Ok(final_url) => Ok(final_url),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, track_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
