// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use solis_lib::core::{parse_packet, TelemetryPacket};
use std::{net::UdpSocket, panic};

fn main() -> std::io::Result<()> {
    solis_lib::run();
    // {
    //     let socket = UdpSocket::bind("127.0.0.1:20777")?;
    //     println!("Binded UDP socket.");
        
    //     let mut buf = [0; 2048];
        
    //     while let Ok((bytes_received, _)) = socket.recv_from(&mut buf) {    
    //         let buf = &mut buf[..bytes_received];
    
    //         println!();

    //         match panic::catch_unwind(|| parse_packet(&buf)) {
    //             Ok(Some(packet)) => {
    //                 match packet.packet_id() {
    //                     0 => {
    //                         if let Some(motion) = packet.as_motion() {
    //                             println!("{:#?}", motion);
    //                         }
    //                     }
    //                     1 => {
    //                         if let Some(session) = packet.as_session() {
    //                             println!("{:#?}", session);
    //                         }
    //                     }
    //                     2 => {
    //                         if let Some(lap_data) = packet.as_lap_data() {
    //                             println!("{:#?}", lap_data);
    //                         }
    //                     }
    //                     3 => {
    //                         if let Some(event) = packet.as_event() {
    //                             println!("{} {}", packet.name(), String::from_utf8_lossy(&event.event_string_code));
    //                         }
    //                     }
    //                     4 => {
    //                         if let Some(participants) = packet.as_participants() {
    //                             println!("{:#?}", participants);
    //                         }
    //                     }
    //                     5 => {
    //                         if let Some(car_setups) = packet.as_car_setups() {
    //                             println!("{:#?}", car_setups);
    //                         }
    //                     }
    //                     6 => {
    //                         if let Some(car_telemetry) = packet.as_car_telemetry() {
    //                             println!("{:#?}", car_telemetry);
    //                         }
    //                     }
    //                     7 => {
    //                         if let Some(car_status) = packet.as_car_status() {
    //                             println!("{:#?}", car_status);
    //                         }
    //                     }
    //                     8 => {
    //                         if let Some(final_classification) = packet.as_final_classification() {
    //                             println!("{:#?}", final_classification);
    //                         }
    //                     }
    //                     9 => {
    //                         if let Some(lobby_info) = packet.as_lobby_info() {
    //                             println!("{:#?}", lobby_info);
    //                         }
    //                     }
    //                     10 => {
    //                         if let Some(car_damage) = packet.as_car_damage() {
    //                             println!("{:#?}", car_damage);
    //                         }
    //                     }
    //                     11 => {
    //                         if let Some(session_history) = packet.as_session_history() {
    //                             println!("{:#?}", session_history);
    //                         }
    //                     }
    //                     _ => {
    //                         println!("Unknown packet id");
    //                     }
    //                 }
    //             }
    //             Ok(None) => println!("parse_packet() => none"),
    //             Err(_) => println!("panic occurred during parse_packet"),
    //         }
    //     }
    // }

    Ok(())
}