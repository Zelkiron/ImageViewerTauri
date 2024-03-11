// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![open_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
#[tauri::command]
fn open_file() {
  //Command::new("explorer").arg(".").spawn().unwrap();
  let file = open::that(".");
}
