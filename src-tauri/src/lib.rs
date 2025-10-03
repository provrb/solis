pub mod audio;
pub mod bridge;
pub mod core;
pub mod strategy;

use crate::{
    bridge::events::{
        get_input_devices, get_output_devices, set_input_device, set_input_volume,
        set_output_volume, start_audio_recording, start_udp_listener, stop_audio_recording,
    },
    core::TelemetryPacket,
    strategy::{answer_question, init},
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            start_udp_listener,
            start_audio_recording,
            stop_audio_recording,
            set_input_volume,
            set_output_volume,
            get_output_devices,
            get_input_devices,
            set_input_device
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
