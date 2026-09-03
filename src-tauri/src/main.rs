// The window and its remote URL are declared in tauri.conf.json, so there is no
// imperative window setup here. windows_subsystem = "windows" keeps a stray
// console from opening alongside the app on Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running Zekoro");
}
