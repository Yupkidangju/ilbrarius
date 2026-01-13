pub mod crawler;
pub mod printer;
pub mod store;

use tauri::Manager;
use crate::crawler::{normalize_url, start_bfs_crawl};

#[tauri::command]
async fn start_crawl(app: tauri::AppHandle, url: String, depth: u32) -> Result<(), String> {
    crate::crawler::start_bfs_crawl(app, url, depth).await.map_err(|e| e.to_string())
}

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
        .setup(|app| {
            // 초기화 작업
            let handle = app.handle().clone();
            tokio::spawn(async move {
                crate::store::init().await;
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, track_url, start_crawl])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
