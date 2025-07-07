use crate::core::ids::{EventId, PenaltyType};
use crate::core::parse_packet;
use crate::{bridge::DataRow, core::PacketParticipantsData};
use std::time::Duration;
use std::{
    net::UdpSocket,
    sync::{Arc, Mutex},
    thread,
};
use tauri::{AppHandle, Emitter, Error};

macro_rules! send_packet_to_buffer {
    ($buffer:expr, $packet_title:tt, $packet_broad:tt, $packet_specific:tt) => {{
        let payload = DataRow {
            title: $packet_title,
            row_title: $packet_broad.name().to_string(),
            timestamp: $packet_broad.session_time().to_string(),
            packet_id: $packet_broad.packet_id().to_string(),
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

static CONNECTED: Mutex<bool> = Mutex::new(false);
fn set_connection_status(new_status: bool) {
    if let Ok(mut con) = CONNECTED.lock() {
        *con = new_status;
    }
}

fn get_connection_status() -> bool {
    CONNECTED.try_lock().map_or(false, |con| *con)
}

#[tauri::command]
pub fn start_udp_listener(app: AppHandle, address: String, port: String) -> Result<bool, Error> {
    let app = app.clone();
    let buffer = Arc::new(Mutex::new(Vec::new()));
    let buffer_clone = buffer.clone();

    let app_clone = app.clone();
    thread::spawn(move || {
        let socket = match UdpSocket::bind(format!("{}:{}", address, port)).ok() {
            Some(socket) => {
                set_connection_status(true);
                println!("[backend] bind {}", get_connection_status());
                let _ = socket.set_read_timeout(Some(Duration::from_millis(500)));

                socket
            }
            None => {
                set_connection_status(false);
                eprintln!("[backend] Binding UDP socket failed.");
                return;
            }
        };

        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(25));
            let mut batch = buffer_clone.lock().unwrap();
            if !batch.is_empty() {
                let to_send = batch.clone();
                batch.clear();
                let _ = app_clone.emit("createDataRowBatch", to_send);
            }
        });

        let mut buf = [0; 2048];
        let mut participant_data = PacketParticipantsData::default();
        loop {
            if !get_connection_status() {
                drop(socket);
                return;
            }
            match socket.recv_from(&mut buf) {
                Ok((bytes_received, _)) => {
                    let buf = &mut buf[..bytes_received];
                    let packet = match parse_packet(buf) {
                        Some(packet) => packet,
                        None => continue,
                    };
                    match packet.packet_id() {
                        0 => {
                            if let Some(motion) = packet.as_motion() {
                                send_packet_to_buffer!(buffer, "Motion Data", packet, motion)
                            }
                        }
                        1 => {
                            if let Some(session) = packet.as_session() {
                                send_packet_to_buffer!(buffer, "Session Data", packet, session)
                            }
                        }
                        2 => {
                            if let Some(lap_data) = packet.as_lap_data() {
                                send_packet_to_buffer!(buffer, "Lap Data", packet, lap_data)
                            }
                        }
                        3 => {
                            if let Some(event) = packet.as_event() {
                                let event_details = match event.event() {
                                    Some(e) => e,
                                    None => return,
                                };

                                let event_data = event.event_details;
                                let raw_data: String = match event_details.id {
                                    EventId::FastestLap => {
                                        let fastest_lap = unsafe { event_data.fastest_lap };
                                        let vehicle_idx = fastest_lap.vehicle_idx;
                                        let lap_time = fastest_lap.lap_time;

                                        if let Some(player) =
                                            participant_data.get_participant(vehicle_idx)
                                        {
                                            format!(
                                                "Fastest lap of {} seconds achieved by {}",
                                                lap_time,
                                                player.get_player_name()
                                            )
                                        } else {
                                            format!("Fastest lap of {} seconds achieved", lap_time)
                                        }
                                    }
                                    EventId::Retirement => {
                                        let retirement = unsafe { event_data.retirement };
                                        let vehicle_idx = retirement.vehicle_idx;

                                        if let Some(player) =
                                            participant_data.get_participant(vehicle_idx)
                                        {
                                            format!(
                                                "{} retired from the session",
                                                player.get_player_name()
                                            )
                                        } else {
                                            "A participant retired from the session".to_string()
                                        }
                                    }
                                    EventId::TeamMateInPits => {
                                        let pits = unsafe { event_data.teammate_in_pits };
                                        let vehicle_idx = pits.vehicle_idx;

                                        if let Some(player) =
                                            participant_data.get_participant(vehicle_idx)
                                        {
                                            format!(
                                                "Your teammate, {}, is in the pits.",
                                                player.get_player_name()
                                            )
                                        } else {
                                            "Your teammate is in the pits".to_string()
                                        }
                                    }
                                    EventId::RaceWinner => {
                                        let race_winner = unsafe { event_data.race_winner };
                                        let vehicle_idx = race_winner.vehicle_idx;

                                        if let Some(player) =
                                            participant_data.get_participant(vehicle_idx)
                                        {
                                            format!(
                                                "{} is the Race Winner!",
                                                player.get_player_name()
                                            )
                                        } else {
                                            event_details.description.to_string()
                                        }
                                    }
                                    EventId::PenaltyIssued => {
                                        let penalty = unsafe { event_data.penalty };
                                        let prim_vehicle_idx = penalty.vehicle_idx;
                                        let lap = penalty.lap_num;
                                        let time = penalty.time;
                                        let penalty_type = penalty.penalty_type;
                                        let infringement = penalty.infringement_type;

                                        if let Some(player) =
                                            participant_data.get_participant(prim_vehicle_idx)
                                        {
                                            if penalty_type == PenaltyType::Retired {
                                                format!("{} has been retired from the session on lap {}.", player.get_player_name(), lap)
                                            } else {
                                                format!("{} has received a {} on lap {}. Infringment type: {:?}. Time: {} seconds", player.get_player_name(), penalty_type.as_str(),  lap, infringement, time)
                                            }
                                        } else {
                                            event_details.description.to_string()
                                        }
                                    }
                                    EventId::SpeedTrapTriggered => {
                                        let speed_trap = unsafe { event_data.speed_trap };
                                        let vehicle_idx = speed_trap.vehicle_idx;
                                        let speed = speed_trap.speed;
                                        let max_speed = speed_trap.fastest_speed_in_session;

                                        if let Some(player) =
                                            participant_data.get_participant(vehicle_idx)
                                        {
                                            if speed_trap.is_overall_fastest_in_session == 1 {
                                                format!("Speed trap hit - {} hit the fastest speed in the session: {} kmh", player.get_player_name(), speed)
                                            } else if speed_trap.is_driver_fastest_in_session == 1 {
                                                format!("Speed trap hit - {} hit their fastest speed in the session: {} kmh", player.get_player_name(), speed)
                                            } else {
                                                format!(
                                                    "Speed trap hit - {} hit {} kmh. Max: {} kmh",
                                                    player.get_player_name(),
                                                    speed,
                                                    max_speed
                                                )
                                            }
                                        } else {
                                            format!(
                                                "Speed trap hit at {} kmh. Max: {} kmh",
                                                speed, max_speed
                                            )
                                        }
                                    }
                                    EventId::StartLights => {
                                        let start_lights = unsafe { event_data.start_lights };
                                        let num = start_lights.num_lights;
                                        format!("{} start lights showing", num)
                                    }
                                    EventId::DriveThroughServed => {
                                        let drive =
                                            unsafe { event_data.drive_through_penalty_served };
                                        let vehicle_idx = drive.vehicle_idx;
                                        if let Some(player) =
                                            participant_data.get_participant(vehicle_idx)
                                        {
                                            format!(
                                                "{} served their drive through penalty",
                                                player.get_player_name()
                                            )
                                        } else {
                                            event_details.description.to_string()
                                        }
                                    }
                                    EventId::StopGoServed => {
                                        let stop_go = unsafe { event_data.stop_go_penalty_served };
                                        let vehicle_idx = stop_go.vehicle_idx;
                                        if let Some(player) =
                                            participant_data.get_participant(vehicle_idx)
                                        {
                                            format!(
                                                "{} served their stop-go penalty",
                                                player.get_player_name()
                                            )
                                        } else {
                                            event_details.description.to_string()
                                        }
                                    }
                                    EventId::Flashback => {
                                        let flashback = unsafe { event_data.flashback };
                                        let session_time = flashback.flashback_session_time;
                                        format!(
                                            "Flashback at {} seconds (session time)",
                                            session_time
                                        )
                                    }
                                    EventId::ButtonStatus => {
                                        let buttons = unsafe { event_data.buttons };
                                        let pressed = buttons.get_pressed_buttons();
                                        if pressed.is_empty() {
                                            continue;
                                        }

                                        format!(
                                            "Button pressed. Pressed buttons: {}",
                                            pressed
                                                .iter()
                                                .map(|button| button.as_str())
                                                .collect::<Vec<_>>()
                                                .join(", ")
                                        )
                                    }
                                    _ => event_details.description.to_string(),
                                };

                                let payload = DataRow {
                                    title: "Events",
                                    row_title: format!(
                                        "{} ({})",
                                        event.event_name(),
                                        event.code_as_string()
                                    ),
                                    timestamp: packet.session_time().to_string(),
                                    packet_id: packet.packet_id().to_string(),
                                    raw_data,
                                };

                                let mut buf = buffer.lock().unwrap();
                                buf.push(payload);
                                if buf.len() > 250 {
                                    let excess = buf.len() - 250;
                                    buf.drain(0..excess);
                                }
                            }
                        }
                        4 => {
                            if let Some(participants) = packet.as_participants() {
                                send_packet_to_buffer!(
                                    buffer,
                                    "Participants",
                                    packet,
                                    participants
                                );

                                // Update cached participant data.
                                if participant_data.num_active_cars != participants.num_active_cars
                                {
                                    participant_data = *participants;
                                }
                            }
                        }
                        5 => {
                            if let Some(car_setups) = packet.as_car_setups() {
                                send_packet_to_buffer!(buffer, "Car Setups", packet, car_setups)
                            }
                        }
                        6 => {
                            if let Some(car_telemetry) = packet.as_car_telemetry() {
                                send_packet_to_buffer!(
                                    buffer,
                                    "Car Telemtry",
                                    packet,
                                    car_telemetry
                                )
                            }
                        }
                        7 => {
                            if let Some(car_status) = packet.as_car_status() {
                                send_packet_to_buffer!(buffer, "Car Status", packet, car_status)
                            }
                        }
                        8 => {
                            if let Some(final_classification) = packet.as_final_classification() {
                                send_packet_to_buffer!(
                                    buffer,
                                    "Final Classification",
                                    packet,
                                    final_classification
                                )
                            }
                        }
                        9 => {
                            if let Some(lobby_info) = packet.as_lobby_info() {
                                send_packet_to_buffer!(buffer, "Lobby Info", packet, lobby_info)
                            }
                        }
                        10 => {
                            if let Some(car_damage) = packet.as_car_damage() {
                                send_packet_to_buffer!(buffer, "Car Damage", packet, car_damage)
                            }
                        }
                        11 => {
                            if let Some(session_history) = packet.as_session_history() {
                                send_packet_to_buffer!(
                                    buffer,
                                    "Session History",
                                    packet,
                                    session_history
                                )
                            }
                        }
                        _ => {
                            println!("Unknown packet id");
                        }
                    }
                }
                Err(ref e)
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.kind() == std::io::ErrorKind::TimedOut =>
                {
                    continue;
                }
                Err(e) => {
                    eprintln!("[backend] UDP socket error: {:?}", e);
                    break;
                }
            }
        }
    });

    thread::sleep(Duration::from_millis(50));

    Ok(get_connection_status())
}

#[tauri::command]
pub fn stop_udp_listener() {
    set_connection_status(false);
}
