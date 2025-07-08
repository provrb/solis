use crate::bridge::DataRow;
use crate::core::ids::PacketType;
use crate::core::Session;
use std::time::Duration;
use std::{
    sync::{Arc, Mutex},
    thread,
};
use tauri::{AppHandle, Emitter, Error, Listener};

macro_rules! send_packet_to_buffer {
    ($buffer:expr, $packet_title:tt, $packet_broad:tt, $packet_specific:tt) => {{
        let payload = DataRow {
            title: $packet_title,
            row_title: $packet_broad.name().to_string(),
            timestamp: $packet_broad.session_time().to_string(),
            packet_id: $packet_broad.packet_id().as_u8().to_string(),
            raw_data: serde_json::to_string_pretty($packet_specific)
                .unwrap_or("Serializing failed for packet.".to_string()),
        };

        let mut buf = $buffer.lock().unwrap();
        buf.push(payload);
        if buf.len() > 250 {
            let excess = buf.len() - 250;
            buf.drain(0..excess);
        }
    }};
}

#[tauri::command]
pub fn start_udp_listener(app: AppHandle, address: String, port: String) -> Result<bool, Error> {
    let app_arc = Arc::new(app);
    let app_clone = Arc::clone(&app_arc);

    // Empty batch of DataRow every 25 seconds.
    // A batch of DataRows containing info about packets will build up
    // and then be sent to the frontned.
    let buffer: Arc<Mutex<Vec<DataRow>>> = Arc::new(Mutex::new(Vec::new()));
    let buffer_clone = buffer.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(25));
        let mut batch = buffer_clone.lock().unwrap();
        if !batch.is_empty() {
            let to_send = batch.clone();
            batch.clear();
            let _ = app_clone.emit("createDataRowBatch", to_send);
        }
    });

    let session = Session::new(address, port);
    let session_clone = Arc::clone(&session);

    // UDP 'Connection' in the below context just means
    // the underlying Session.connection struct has an active
    // UDP socket.

    // Thread to listen for the stop_udp_listener event from
    // the frontend. Will cut the session UDP 'connection', causing
    // the thread receiving packets to break, making the backend ready
    // to establish a fresh, new connection later on.
    thread::spawn(move || {
        app_arc.listen("stop_udp_listener", move |_| {
            if let Ok(mut session) = session_clone.lock() {
                session.drop_connection();
            }
        })
    });

    // Thread that will get the lastest telemtry packet with an established
    // UDP 'connection' and then dispatch the neccessary info to the frontend
    // to be displayed as a data row
    thread::spawn(move || loop {
        // Give the 'stop_udp_listener' thread a chance to
        // obtain the lock before this thread.
        thread::sleep(Duration::from_millis(10));

        let session_guard = session.lock().unwrap();
        if !session_guard.connected() {
            println!("Not connected. breaking from thread");
            break;
        }

        let Ok(packet) = session_guard.get_latest_packet() else {
            continue;
        };

        // Dispatch packet data to frontend
        match packet.packet_id() {
            PacketType::Motion => {
                if let Some(motion) = packet.as_motion() {
                    send_packet_to_buffer!(buffer, "Motion Data", packet, motion)
                }
            }
            PacketType::Session => {
                if let Some(session) = packet.as_session() {
                    send_packet_to_buffer!(buffer, "Session Data", packet, session)
                }
            }
            PacketType::LapData => {
                if let Some(lap_data) = packet.as_lap_data() {
                    send_packet_to_buffer!(buffer, "Lap Data", packet, lap_data)
                }
            }
            PacketType::Event => {
                if let Some(event) = packet.as_event() {
                    let payload = DataRow {
                        title: "Events",
                        row_title: format!("{} ({})", event.event_name(), event.code_as_string()),
                        timestamp: packet.session_time().to_string(),
                        packet_id: packet.packet_id().as_u8().to_string(),
                        raw_data: event.event_message(session_guard),
                    };

                    let mut buf = buffer.lock().unwrap();
                    buf.push(payload);
                    if buf.len() > 250 {
                        let excess = buf.len() - 250;
                        buf.drain(0..excess);
                    }
                }
            }
            PacketType::Participants => {
                if let Some(participants) = packet.as_participants() {
                    send_packet_to_buffer!(buffer, "Participants", packet, participants);
                }
            }
            PacketType::CarSetups => {
                if let Some(car_setups) = packet.as_car_setups() {
                    send_packet_to_buffer!(buffer, "Car Setups", packet, car_setups)
                }
            }
            PacketType::CarTelemetry => {
                if let Some(car_telemetry) = packet.as_car_telemetry() {
                    send_packet_to_buffer!(buffer, "Car Telemetry", packet, car_telemetry)
                }
            }
            PacketType::CarStatus => {
                if let Some(car_status) = packet.as_car_status() {
                    send_packet_to_buffer!(buffer, "Car Status", packet, car_status)
                }
            }
            PacketType::FinalClassification => {
                if let Some(final_classification) = packet.as_final_classification() {
                    send_packet_to_buffer!(
                        buffer,
                        "Final Classification",
                        packet,
                        final_classification
                    )
                }
            }
            PacketType::LobbyInfo => {
                if let Some(lobby_info) = packet.as_lobby_info() {
                    send_packet_to_buffer!(buffer, "Lobby Info", packet, lobby_info)
                }
            }
            PacketType::CarDamage => {
                if let Some(car_damage) = packet.as_car_damage() {
                    send_packet_to_buffer!(buffer, "Car Damage", packet, car_damage)
                }
            }
            PacketType::SessionHistory => {
                if let Some(session_history) = packet.as_session_history() {
                    send_packet_to_buffer!(buffer, "Session History", packet, session_history)
                }
            }
            _ => (),
        }
    });

    thread::sleep(Duration::from_millis(100));

    Ok(true)
}
