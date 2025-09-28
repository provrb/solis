use crate::core::{ids::PacketType, parse_packet, ParticipantData, TelemetryPacket};
use socket2::{Domain, Socket, Type};
use std::{
    mem::MaybeUninit,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};

/// This struct provides detailed information
/// on a Socket connection like whether or not the connection is
/// active, the address and port, and the underlying Socket type
/// If 'socket' is None, then no connection is established
#[derive(Debug, Default)]
pub struct Connection {
    pub active: bool,
    pub address: String,
    /// The address the socket is bound to.
    /// e.g 127.0.0.1
    pub port: String,
    pub socket: Option<Socket>,
}

impl Connection {
    /// Create and bind a UDP socket to the specified IPV4 address and port using the
    /// Socket2 crate.
    /// Toggles different socket options such as:
    /// SO_REUSEADDR, read and write timeout
    pub fn new(address: String, port: String, timeout: Option<Duration>) -> Self {
        let sfd = match Socket::new(Domain::IPV4, Type::DGRAM, None) {
            Ok(s) => Some(s),
            Err(e) => {
                println!("Error creating DGRAM socket: {e}");
                None
            }
        };

        let mut active = false;
        if let Some(socket) = &sfd {
            // Set socket options
            let _ = socket.set_reuse_address(true);
            let _ = socket.set_read_timeout(timeout);
            let _ = socket.set_write_timeout(timeout);

            // Create socket address
            // e.g 127.0.0.1:20777
            let full_address = format!("{}:{}", address, port);
            let socket_addr: SocketAddr = full_address.parse().unwrap();

            // Bind UDP socket. Active will be true if successful, otherwise false.
            active = match socket.bind(&socket_addr.into()) {
                Ok(_) => {
                    println!("Binded UDP socket");
                    true
                }
                Err(e) => {
                    println!("Error binding UDP socket: {e}");
                    false
                }
            }
        }

        Self {
            address,
            port,
            socket: sfd,
            active,
        }
    }

    /// Check if the current connection is active.
    /// Checks the 'active' boolean and if 'socket' is not None
    pub fn is_active(&self) -> bool {
        self.active && self.socket.as_ref().is_some()
    }
}

/// A struct describing an F1 session with a telemetry connection active.
///
/// Contains information about the players and number of active cars from
/// the Participant Data packet.
#[derive(Debug, Default)]
pub struct Session {
    pub connection: Connection,
    pub max_packet_buffer_len: u16,
    pub num_active_cars: u8,
    /// A list of all current players
    /// Accessed by the players vehicle index which can be found
    /// from other packets & events.
    pub players: [ParticipantData; 22],
}

impl Session {
    /// Create a new telemetry connection for an F1 session.
    ///
    /// Attempts to asynchronously and automatically get information on all players in the session
    /// to fill out the 'players' field will valid data.
    pub fn new(address: String, port: String) -> Arc<Mutex<Self>> {
        let session = Arc::new(Mutex::new(Session {
            connection: Connection::new(address, port, Some(Duration::from_millis(500))),
            max_packet_buffer_len: 250,
            num_active_cars: 0,
            players: Default::default(),
        }));

        if session.lock().unwrap().connection.is_active() {
            let session_clone = Arc::clone(&session);
            tauri::async_runtime::spawn(async move { Session::get_players(session_clone).await });
        }

        session
    }

    /// Set the 'active' flag in the 'connection' field to
    /// false, telling all socket operation dependant functions in Session or Connection
    /// to stop.
    pub fn drop_connection(&mut self) {
        println!("Dropped connection");
        self.connection.active = false;
        self.connection.socket = None;
    }

    /// Asynchronously get information on all players in the session specified.
    ///
    /// The session must have an active telemetry UDP socket. This function will receive
    /// all telemtry packets until it receives a valid PacketParticipantData, however, it
    /// is assumed that the socket has a read timeout, if constructed with Connection::new(), it will.
    ///
    /// Extracts the 'num_active_cars' and 'participants' field and saves it to
    /// the 'players' field of 'session'
    pub async fn get_players(session: Arc<Mutex<Session>>) {
        let socket = {
            let guard = session.lock().unwrap();
            match &guard.connection.socket {
                Some(s) => s.try_clone().ok(), // clone so we don’t hold the lock
                None => None,
            }
        };

        let Some(socket) = socket else { return };

        let mut buf = [MaybeUninit::<u8>::uninit(); 2048];
        loop {
            match socket.recv_from(&mut buf) {
                Ok((bytes_received, _)) => {
                    // Check if the packet received is the packet containing
                    // all the participant data. If it is, update the session
                    // players array and the number of players

                    let initialized = unsafe {
                        std::slice::from_raw_parts(buf.as_ptr() as *const u8, bytes_received)
                    };
                    let packet = match parse_packet(initialized) {
                        Some(packet) => packet,
                        None => continue,
                    };

                    // Checking the packet_id in the packet header first is
                    // less performance heavy than attempting to interpret the packet
                    // as PacketParticipantData
                    if packet.packet_id() == PacketType::Participants {
                        if let Some(packet) = packet.as_participants() {
                            let mut session_guard = session.lock().unwrap();
                            session_guard.players = packet.participants;
                            session_guard.num_active_cars = packet.num_active_cars;

                            println!("[backend] Player information loaded.");
                            break;
                        }
                    }
                }
                Err(ref e)
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.kind() == std::io::ErrorKind::TimedOut =>
                {
                    continue
                }
                Err(_) => break,
            }
        }
    }

    /// Get information on a player from their vehicle index.
    ///
    /// If the player with the vehicle index does not exist, return None.
    /// A player's vehicle index can be found from events and other packets.
    pub fn get_player(&self, plr_vehicle_idx: u8) -> Option<&ParticipantData> {
        self.players.get(plr_vehicle_idx as usize)
    }

    /// Receive the latest F1 telemetry packet
    pub fn get_latest_packet(&self) -> Option<TelemetryPacket> {
        let mut buf = [MaybeUninit::<u8>::uninit(); 2048];
        let socket = match &self.connection.socket {
            Some(s) => s,
            None => return None,
        };

        match socket.recv_from(&mut buf) {
            Ok((bytes_received, _)) => {
                let initialized = unsafe {
                    std::slice::from_raw_parts(buf.as_ptr() as *const u8, bytes_received)
                };
                let packet = parse_packet(initialized)?;
                Some(packet)
            }
            Err(_) => None,
        }
    }

    /// Check if a session has an active UDP socket
    pub fn connected(&self) -> bool {
        self.connection.is_active()
    }
}
