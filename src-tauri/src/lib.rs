pub mod audio;
pub mod bridge;
pub mod core;

use crate::bridge::events::{start_audio_recording, start_udp_listener, stop_audio_recording};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            start_udp_listener,
            start_audio_recording,
            stop_audio_recording
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
