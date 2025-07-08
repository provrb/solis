use std::sync::MutexGuard;

use crate::core::{
    cm_events::{Event, CM_EVENTS},
    ids::{ButtonFlag, EventId, PacketType},
    Session,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// F1 22 Packet Definitions
/// Provided by CodeMaster
///
/// All structs are packed. Everything in little endian.
/// Sent over UDP (telemetry) and then to be interpreted
/// by Solis Core.
///
/// Note: All wheel arrays have the following order:
/// Index - Name
///   0   - Rear Left (RL)
///   1   - Rear Right (RR)
///   2   - Front Left (FL)
///   3   - Front Right (FR)
use crate::core::ids::{
    DriverId, FormulaType, GameModeId, InfringementType, NationalityId, PenaltyType, RulesetId,
    SessionLength, SessionType, SurfaceType, TeamId, TrackId, WeatherType,
};

/// Every packet will have the following header.
#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct PacketHeader {
    pub packet_format: u16,
    pub game_major_version: u8,
    pub game_minor_version: u8,
    pub packet_version: u8,

    /// Identifier for the packet type, see below
    ///
    /// | Packet Name           | Val | Description                                                                                  |
    /// |-----------------------|-----|----------------------------------------------------------------------------------------------|
    /// | Motion                | 0   | Contains all motion data for player’s car – only sent while player is in control             |
    /// | Session               | 1   | Data about the session – track, time left                                                    |
    /// | Lap Data              | 2   | Data about all the lap times of cars in the session                                          |
    /// | Event                 | 3   | Various notable events that happen during a session                                          |
    /// | Participants          | 4   | List of participants in the session, mostly relevant for multiplayer                         |
    /// | Car Setups            | 5   | Packet detailing car setups for cars in the race                                             |
    /// | Car Telemetry         | 6   | Telemetry data for all cars                                                                  |
    /// | Car Status            | 7   | Status data for all cars                                                                     |
    /// | Final Classification  | 8   | Final classification confirmation at the end of a race                                       |
    /// | Lobby Info            | 9   | Information about players in a multiplayer lobby                                             |
    /// | Car Damage            | 10  | Damage status for all cars                                                                   |
    /// | Session History       | 11  | Lap and tyre data for session                                                                |
    pub packet_id: PacketType,
    pub session_uid: u64,
    pub session_time: f32,
    pub frame_identifier: u32,
    pub player_car_index: u8,

    /// Index of secondary player's car in the array (splitscreen)
    /// 255 if no second player
    pub secondary_player_car_index: u8,
}

/// Physics data for a vehicle
#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
struct CarMotionData {
    world_position_x: f32,     // World space X position
    world_position_y: f32,     // World space Y position
    world_position_z: f32,     // World space Z position
    world_velocity_x: f32,     // Velocity in world space X
    world_velocity_y: f32,     // Velocity in world space Y
    world_velocity_z: f32,     // Velocity in world space Z
    world_forward_dir_x: i16,  // World space forward X direction (normalised)
    world_forward_dir_y: i16,  // World space forward Y direction (normalised)
    world_forward_dir_z: i16,  // World space forward Z direction (normalised)
    world_right_dir_x: i16,    // World space right X direction (normalised)
    world_right_dir_y: i16,    // World space right Y direction (normalised)
    world_right_dir_z: i16,    // World space right Z direction (normalised)
    g_force_lateral: f32,      // Lateral G-Force component
    g_force_longitudinal: f32, // Longitudinal G-Force component
    g_force_vertical: f32,     // Vertical G-Force component
    yaw: f32,                  // Yaw angle in radians
    pitch: f32,                // Pitch angle in radians
    roll: f32,                 // Roll angle in radians
}

/// Physics data for all the cars being driven.
///
/// Includes additional data for the car being driven
/// with the goal of being able to drive a motion platform setup.
#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PacketMotionData {
    pub header: PacketHeader,             // Header
    car_motion_data: [CarMotionData; 22], // Data for all cars on track

    // Extra player car ONLY data
    pub suspension_velocity: [f32; 4],     // RL, RR, FL, FR
    pub suspension_acceleration: [f32; 4], // RL, RR, FL, FR
    pub suspension_position: [f32; 4],     // RL, RR, FL, FR
    pub wheel_speed: [f32; 4],             // Speed of each wheel
    pub wheel_slip: [f32; 4],              // Slip ratio for each wheel
    pub local_velocity_x: f32,             // Velocity in local space
    pub local_velocity_y: f32,             // Velocity in local space
    pub local_velocity_z: f32,             // Velocity in local space
    pub angular_velocity_x: f32,           // Angular velocity x-component
    pub angular_velocity_y: f32,           // Angular velocity y-component
    pub angular_velocity_z: f32,           // Angular velocity z-component
    pub angular_acceleration_x: f32,       // Angular velocity x-component
    pub angular_acceleration_y: f32,       // Angular velocity y-component
    pub angular_acceleration_z: f32,       // Angular velocity z-component
    pub front_wheels_angle: f32,           // Current front wheels angle in radians
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub struct MarshalZone {
    zone_start: f32, // Fraction (0..1) of way through the lap the marshal zone starts
    zone_flag: i8,   // -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow, 4 = red
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub struct WeatherForecastSample {
    session_type: SessionType,
    time_offset: u8, // Time in minutes the forecast is for
    weather: WeatherType,
    track_temperature: i8,        // Track temp. in degrees Celsius
    track_temperature_change: i8, // Track temp. change – 0 = up, 1 = down, 2 = no change
    air_temperature: i8,          // Air temp. in degrees celsius
    air_temperature_change: i8,   // Air temp. change – 0 = up, 1 = down, 2 = no change
    rain_percentage: u8,          // Rain percentage (0-100)
}

#[repr(C, packed)]
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PacketSessionData {
    pub header: PacketHeader, // Header
    pub weather: WeatherType,
    pub track_temperature: i8, // Track temp. in degrees celsius
    pub air_temperature: i8,   // Air temp. in degrees celsius
    pub total_laps: u8,        // Total number of laps in this race
    pub track_length: u16,     // Track length in metres
    pub session_type: SessionType,
    pub track_id: TrackId,
    pub formula: FormulaType,
    pub session_time_left: u16,     // Time left in session in seconds
    pub session_duration: u16,      // Session duration in seconds
    pub pit_speed_limit: u8,        // Pit speed limit in kilometres per hour
    pub game_paused: u8,            // Whether the game is paused – network game only
    pub is_spectating: u8,          // Whether the player is spectating
    pub spectator_car_index: u8,    // Index of the car being spectated
    pub sli_pro_native_support: u8, // SLI Pro support, 0 = inactive, 1 = active
    pub num_marshal_zones: u8,      // Number of marshal zones to follow
    #[serde_as(as = "[_; 21]")]
    marshal_zones: [MarshalZone; 21], // List of marshal zones – max 21
    pub safety_car_status: u8,      // 0 = no safety car, 1 = full
    // 2 = virtual, 3 = formation lap
    pub network_game: u8,                 // 0 = offline, 1 = online
    pub num_weather_forecast_samples: u8, // Number of weather samples to follow
    #[serde_as(as = "[_; 56]")]
    weather_forecast_samples: [WeatherForecastSample; 56], // Array of weather forecast samples
    pub forecast_accuracy: u8,            // 0 = Perfect, 1 = Approximate
    pub ai_difficulty: u8,                // AI Difficulty rating – 0-110
    pub season_link_identifier: u32,      // Identifier for season - persists across saves
    pub weekend_link_identifier: u32,     // Identifier for weekend - persists across saves
    pub session_link_identifier: u32,     // Identifier for session - persists across saves
    pub pit_stop_window_ideal_lap: u8,    // Ideal lap to pit on for current strategy (player)
    pub pit_stop_window_latest_lap: u8,   // Latest lap to pit on for current strategy (player)
    pub pit_stop_rejoin_position: u8,     // Predicted position to rejoin at (player)
    pub steering_assist: u8,              // 0 = off, 1 = on
    pub braking_assist: u8,               // 0 = off, 1 = low, 2 = medium, 3 = high
    pub gearbox_assist: u8,               // 1 = manual, 2 = manual & suggested gear, 3 = auto
    pub pit_assist: u8,                   // 0 = off, 1 = on
    pub pit_release_assist: u8,           // 0 = off, 1 = on
    pub ers_assist: u8,                   // 0 = off, 1 = on
    pub drs_assist: u8,                   // 0 = off, 1 = on
    pub dynamic_racing_line: u8,          // 0 = off, 1 = corners only, 2 = full
    pub dynamic_racing_line_type: u8,     // 0 = 2D, 1 = 3D
    pub game_mode: GameModeId,            // Game mode id - see appendix
    pub rule_set: RulesetId,              // Ruleset - see appendix
    pub time_of_day: u32,                 // Local time of day - minutes since midnight
    pub session_length: SessionLength,
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct LapData {
    last_lap_time_in_ms: u32,            // Last lap time in milliseconds
    current_lap_time_in_ms: u32,         // Current time around the lap in milliseconds
    sector1_time_in_ms: u16,             // Sector 1 time in milliseconds
    sector2_time_in_ms: u16,             // Sector 2 time in milliseconds
    lap_distance: f32, // Distance vehicle is around current lap in metres – could be negative if line hasn’t been crossed yet
    total_distance: f32, // Total distance travelled in session in metres – could be negative if line hasn’t been crossed yet
    safety_car_delta: f32, // Delta in seconds for safety car
    car_position: u8,    // Car race position
    current_lap_num: u8, // Current lap number
    pit_status: u8,      // 0 = none, 1 = pitting, 2 = in pit area
    num_pit_stops: u8,   // Number of pit stops taken in this race
    sector: u8,          // 0 = sector1, 1 = sector2, 2 = sector3
    current_lap_invalid: u8, // Current lap invalid - 0 = valid, 1 = invalid
    penalties: u8,       // Accumulated time penalties in seconds to be added
    warnings: u8,        // Accumulated number of warnings issued
    num_unserved_drive_through_pens: u8, // Num drive through pens left to serve
    num_unserved_stop_go_pens: u8, // Num stop go pens left to serve
    grid_position: u8,   // Grid position the vehicle started the race in
    driver_status: u8, // Status of driver - 0 = in garage, 1 = flying lap, 2 = in lap, 3 = out lap, 4 = on track
    result_status: u8, // Result status - 0 = invalid, 1 = inactive, 2 = active, 3 = finished, 4 = didnotfinish, 5 = disqualified, 6 = not classified, 7 = retired
    pit_lane_timer_active: u8, // Pit lane timing, 0 = inactive, 1 = active
    pit_lane_time_in_lane_in_ms: u16, // If active, the current time spent in the pit lane in ms
    pit_stop_timer_in_ms: u16, // Time of the actual pit stop in ms
    pit_stop_should_serve_pen: u8, // Whether the car should serve a penalty at this stop
}

/// The lap data packet gives details of all the cars in the session.
#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PacketLapData {
    pub header: PacketHeader,
    lap_data: [LapData; 22],          // Lap data for all cars on track
    pub time_trial_pb_car_idx: u8,    // Index of Personal Best car in time trial (255 if invalid)
    pub time_trial_rival_car_idx: u8, // Index of Rival car in time trial (255 if invalid)
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FastestLap {
    pub vehicle_idx: u8, // Vehicle index of car achieving fastest lap
    pub lap_time: f32,   // Lap time is in seconds
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Retirement {
    pub vehicle_idx: u8, // Vehicle index of car retiring
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct TeamMateInPits {
    pub vehicle_idx: u8, // Vehicle index of team mate
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct RaceWinner {
    pub vehicle_idx: u8, // Vehicle index of the race winner
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Penalty {
    pub penalty_type: PenaltyType, // Penalty type – see Appendices
    pub infringement_type: InfringementType, // Infringement type – see Appendices
    pub vehicle_idx: u8,           // Vehicle index of the car the penalty is applied to
    pub other_vehicle_idx: u8,     // Vehicle index of the other car involved
    pub time: u8,                  // Time gained, or time spent doing action in seconds
    pub lap_num: u8,               // Lap the penalty occurred on
    pub places_gained: u8,         // Number of places gained by this
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct SpeedTrap {
    pub vehicle_idx: u8, // Vehicle index of the vehicle triggering speed trap
    pub speed: f32,      // Top speed achieved in kilometres per hour
    pub is_overall_fastest_in_session: u8, // Overall fastest speed in session = 1, otherwise 0
    pub is_driver_fastest_in_session: u8, // Fastest speed for driver in session = 1, otherwise 0
    pub fastest_vehicle_idx_in_session: u8, // Vehicle index of the vehicle that is the fastest in this session
    pub fastest_speed_in_session: f32,      // Speed of the vehicle that is the fastest
                                            // in this session
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct StartLights {
    pub num_lights: u8, // Number of lights showing
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct DriveThroughPenaltyServed {
    pub vehicle_idx: u8, // Vehicle index of the vehicle serving drive through
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct StopGoPenaltyServed {
    pub vehicle_idx: u8, // Vehicle index of the vehicle serving stop go
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Flashback {
    pub flashback_frame_identifier: u32, // Frame identifier flashed back to
    pub flashback_session_time: f32,     // Session time flashed back to
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Buttons {
    pub button_status: u32, // Bit flags specifying which buttons are being pressed
                            // currently - see appendices
}

impl Buttons {
    pub fn get_pressed_buttons(&self) -> Vec<ButtonFlag> {
        const ALL_BUTTONS: &[ButtonFlag] = &[
            ButtonFlag::CircleorB,
            ButtonFlag::CrossorA,
            ButtonFlag::DpadDown,
            ButtonFlag::DpadLeft,
            ButtonFlag::DpadRight,
            ButtonFlag::DpadUp,
            ButtonFlag::L1orLB,
            ButtonFlag::L2orLT,
            ButtonFlag::LeftStickClick,
            ButtonFlag::OptionsorMenu,
            ButtonFlag::R1orRB,
            ButtonFlag::R2orRT,
            ButtonFlag::RightStickClick,
            ButtonFlag::RightStickDown,
            ButtonFlag::RightStickLeft,
            ButtonFlag::RightStickRight,
            ButtonFlag::RightStickUp,
            ButtonFlag::Special,
            ButtonFlag::SquareorX,
            ButtonFlag::TriangleorY,
        ];

        // Iterate through each button and check if
        // button_status AND button_flag is set.
        // If set, button is being pressed.
        ALL_BUTTONS
            .iter()
            .copied()
            .filter(|&flag| (self.button_status & flag as u32) != 0)
            .collect()
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub union EventDataDetails {
    pub fastest_lap: FastestLap,
    pub retirement: Retirement,
    pub teammate_in_pits: TeamMateInPits,
    pub race_winner: RaceWinner,
    pub penalty: Penalty,
    pub speed_trap: SpeedTrap,
    pub start_lights: StartLights,
    pub drive_through_penalty_served: DriveThroughPenaltyServed,
    pub stop_go_penalty_served: StopGoPenaltyServed,
    pub flashback: Flashback,
    pub buttons: Buttons,
}

/// This packet gives details of events that happen during the course of a session.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketEventData {
    pub header: PacketHeader,

    /// Event string code - determines the type of event, see below
    ///         Event        -    Code    -          Description
    /// Session Started      -   "SSTA"   -   Sent when the session starts
    /// Session Ended        -   "SEND"   -   Sent when the session ends
    /// Fastest Lap          -   "FTLP"   -   When a driver achieves the fastest lap
    /// Retirement           -   "RTMT"   -   When a driver retires
    /// DRS enabled          -   "DRSE"   -   Race control have enabled DRS
    /// DRS disabled         -   "DRSD"   -   Race control have disabled DRS
    /// Team mate in pits    -   "TMPT"   -   Your team mate has entered the pits
    /// Chequered flag       -   "CHQF"   -   The chequered flag has been waved
    /// Race Winner          -   "RCWN"   -   The race winner is announced
    /// Penalty Issued       -   "PENA"   -   A penalty has been issued – details in event
    /// Speed Trap Triggered -   "SPTP"   -   Speed trap has been triggered by fastest speed
    /// Start lights         -   "STLG"   -   Start lights – number shown
    /// Lights out           -   "LGOT"   -   Lights out
    /// Drive through served -   "DTSV"   -   Drive through penalty served
    /// Stop go served       -   "SGSV"   -   Stop go penalty served
    /// Flashback            -   "FLBK"   -   Flashback activated
    /// Button status        -   "BUTN"   -   Button status changed
    pub event_string_code: [u8; 4],

    pub event_details: EventDataDetails, // Event details - should be interpreted differently
                                         // for each type
}

impl PacketEventData {
    pub fn code_as_string(&self) -> String {
        String::from_utf8_lossy(&self.event_string_code).to_string()
    }

    pub fn event_name(&self) -> String {
        CM_EVENTS
            .get(self.code_as_string().as_str())
            .map(|event| event.event.to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    }

    pub fn event_description(&self) -> String {
        CM_EVENTS
            .get(self.code_as_string().as_str())
            .map(|event| event.description.to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    }

    pub fn event_reference(&self) -> Option<&Event> {
        CM_EVENTS.get(self.code_as_string().as_str())
    }

    pub fn event_message(&self, session: MutexGuard<'_, Session>) -> String {
        let event_details = match self.event_reference() {
            Some(e) => e,
            None => return String::new(),
        };

        let data_details = self.event_details;
        match event_details.id {
            EventId::FastestLap => {
                let fastest_lap = unsafe { data_details.fastest_lap };
                let vehicle_idx = fastest_lap.vehicle_idx;
                let lap_time = fastest_lap.lap_time;

                if let Some(player) = session.get_player(vehicle_idx) {
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
                let retirement = unsafe { data_details.retirement };
                let vehicle_idx = retirement.vehicle_idx;

                if let Some(player) = session.get_player(vehicle_idx) {
                    format!("{} retired from the session", player.get_player_name())
                } else {
                    "A participant retired from the session".to_string()
                }
            }
            EventId::TeamMateInPits => {
                let pits = unsafe { data_details.teammate_in_pits };
                let vehicle_idx = pits.vehicle_idx;

                if let Some(player) = session.get_player(vehicle_idx) {
                    format!(
                        "Your teammate, {}, is in the pits.",
                        player.get_player_name()
                    )
                } else {
                    "Your teammate is in the pits".to_string()
                }
            }
            EventId::RaceWinner => {
                let race_winner = unsafe { data_details.race_winner };
                let vehicle_idx = race_winner.vehicle_idx;

                if let Some(player) = session.get_player(vehicle_idx) {
                    format!("{} is the Race Winner!", player.get_player_name())
                } else {
                    event_details.description.to_string()
                }
            }
            EventId::PenaltyIssued => {
                let penalty = unsafe { data_details.penalty };
                let prim_vehicle_idx = penalty.vehicle_idx;
                let lap = penalty.lap_num;
                let time = penalty.time;
                let penalty_type = penalty.penalty_type;
                let infringement = penalty.infringement_type;

                if let Some(player) = session.get_player(prim_vehicle_idx) {
                    if penalty_type == PenaltyType::Retired {
                        format!(
                            "{} has been retired from the session on lap {}.",
                            player.get_player_name(),
                            lap
                        )
                    } else {
                        format!("{} has received a {} on lap {}. Infringment type: {:?}. Time: {} seconds", player.get_player_name(), penalty_type.as_str(),  lap, infringement, time)
                    }
                } else {
                    event_details.description.to_string()
                }
            }
            EventId::SpeedTrapTriggered => {
                let speed_trap = unsafe { data_details.speed_trap };
                let vehicle_idx = speed_trap.vehicle_idx;
                let speed = speed_trap.speed;
                let max_speed = speed_trap.fastest_speed_in_session;

                if let Some(player) = session.get_player(vehicle_idx) {
                    if speed_trap.is_overall_fastest_in_session == 1 {
                        format!(
                            "Speed trap hit - {} hit the fastest speed in the session: {} kmh",
                            player.get_player_name(),
                            speed
                        )
                    } else if speed_trap.is_driver_fastest_in_session == 1 {
                        format!(
                            "Speed trap hit - {} hit their fastest speed in the session: {} kmh",
                            player.get_player_name(),
                            speed
                        )
                    } else {
                        format!(
                            "Speed trap hit - {} hit {} kmh. Max: {} kmh",
                            player.get_player_name(),
                            speed,
                            max_speed
                        )
                    }
                } else {
                    format!("Speed trap hit at {} kmh. Max: {} kmh", speed, max_speed)
                }
            }
            EventId::StartLights => {
                let start_lights = unsafe { data_details.start_lights };
                let num = start_lights.num_lights;
                format!("{} start lights showing", num)
            }
            EventId::DriveThroughServed => {
                let drive = unsafe { data_details.drive_through_penalty_served };
                let vehicle_idx = drive.vehicle_idx;
                if let Some(player) = session.get_player(vehicle_idx) {
                    format!(
                        "{} served their drive through penalty",
                        player.get_player_name()
                    )
                } else {
                    event_details.description.to_string()
                }
            }
            EventId::StopGoServed => {
                let stop_go = unsafe { data_details.stop_go_penalty_served };
                let vehicle_idx = stop_go.vehicle_idx;
                if let Some(player) = session.get_player(vehicle_idx) {
                    format!("{} served their stop-go penalty", player.get_player_name())
                } else {
                    event_details.description.to_string()
                }
            }
            EventId::Flashback => {
                let flashback = unsafe { data_details.flashback };
                let session_time = flashback.flashback_session_time;
                format!("Flashback at {} seconds (session time)", session_time)
            }
            EventId::ButtonStatus => {
                let buttons = unsafe { data_details.buttons };
                let pressed = buttons.get_pressed_buttons();
                if pressed.is_empty() {
                    return String::new();
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
        }
    }
}

#[repr(C, packed)]
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ParticipantData {
    pub ai_controlled: u8, // Whether the vehicle is AI (1) or Human (0) controlled
    pub driver_id: DriverId, // Driver id - see appendix, 255 if network human
    pub network_id: u8,    // Network id – unique identifier for network players
    pub team_id: TeamId,
    pub my_team: u8,                // My team flag – 1 = My Team, 0 = otherwise
    pub race_number: u8,            // Race number of the car
    pub nationality: NationalityId, // Nationality of the driver

    #[serde_as(as = "[_; 48]")]
    pub name: [u8; 48], // Name of participant in UTF-8 format – null terminated. Truncated with … (U+2026) if too long
    pub your_telemetry: u8, // The player's UDP setting, 0 = restricted, 1 = public
}

impl Default for ParticipantData {
    fn default() -> Self {
        Self {
            name: [0; 48],
            ai_controlled: 0,
            driver_id: DriverId::default(),
            network_id: 0,
            team_id: TeamId::default(),
            my_team: 0,
            race_number: 0,
            nationality: NationalityId::default(),
            your_telemetry: 0,
        }
    }
}

impl ParticipantData {
    pub fn get_player_name(&self) -> String {
        String::from_utf8_lossy(self.name.split(|&b| b == b'\0').nth(0).unwrap_or(&[])).to_string()
    }
}

/// This is a list of participants in the race.
///
/// If the vehicle is controlled by AI, then the name will be the driver name.
///
/// If this is a multiplayer game, the names will be the Steam Id on PC,
/// or the LAN name if appropriate.
///
/// The array should be indexed by vehicle index.
#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct PacketParticipantsData {
    pub header: PacketHeader,
    pub num_active_cars: u8, // Number of active cars in the data – should match number of cars on HUD
    pub participants: [ParticipantData; 22],
}

impl PacketParticipantsData {
    pub fn get_participant(&self, vehicle_idx: u8) -> Option<&ParticipantData> {
        self.participants.get(vehicle_idx as usize)
    }
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct CarSetupData {
    front_wing: u8,                 // Front wing aero
    rear_wing: u8,                  // Rear wing aero
    on_throttle: u8,                // Differential adjustment on throttle (percentage)
    off_throttle: u8,               // Differential adjustment off throttle (percentage)
    front_camber: f32,              // Front camber angle (suspension geometry)
    rear_camber: f32,               // Rear camber angle (suspension geometry)
    front_toe: f32,                 // Front toe angle (suspension geometry)
    rear_toe: f32,                  // Rear toe angle (suspension geometry)
    front_suspension: u8,           // Front suspension
    rear_suspension: u8,            // Rear suspension
    front_anti_roll_bar: u8,        // Front anti-roll bar
    rear_anti_roll_bar: u8,         // Front anti-roll bar
    front_suspension_height: u8,    // Front ride height
    rear_suspension_height: u8,     // Rear ride height
    brake_pressure: u8,             // Brake pressure (percentage)
    brake_bias: u8,                 // Brake bias (percentage)
    rear_left_tyre_pressure: f32,   // Rear left tyre pressure (PSI)
    rear_right_tyre_pressure: f32,  // Rear right tyre pressure (PSI)
    front_left_tyre_pressure: f32,  // Front left tyre pressure (PSI)
    front_right_tyre_pressure: f32, // Front right tyre pressure (PSI)
    ballast: u8,                    // Ballast
    fuel_load: f32,                 // Fuel load
}

/// This packet details the car setups for each vehicle in the session.
///
/// Note that in multiplayer games, other player cars will appear as blank,
/// you will only be able to see your car setup and AI cars.
#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PacketCarSetupData {
    pub header: PacketHeader,
    car_setups: [CarSetupData; 22],
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct CarTelemetryData {
    speed: u16,                         // Speed of car in kilometres per hour
    throttle: f32,                      // Amount of throttle applied (0.0 to 1.0)
    steer: f32,                         // Steering (-1.0 (full lock left) to 1.0 (full lock right))
    brake: f32,                         // Amount of brake applied (0.0 to 1.0)
    clutch: u8,                         // Amount of clutch applied (0 to 100)
    gear: i8,                           // Gear selected (1-8, N=0, R=-1)
    engine_rpm: u16,                    // Engine RPM
    drs: u8,                            // 0 = off, 1 = on
    rev_lights_percent: u8,             // Rev lights indicator (percentage)
    rev_lights_bit_value: u16,          // Rev lights (bit 0 = leftmost LED, bit 14 = rightmost LED)
    brakes_temperature: [u16; 4],       // Brakes temperature (celsius)
    tyres_surface_temperature: [u8; 4], // Tyres surface temperature (celsius)
    tyres_inner_temperature: [u8; 4],   // Tyres inner temperature (celsius)
    engine_temperature: u16,            // Engine temperature (celsius)
    tyres_pressure: [f32; 4],           // Tyres pressure (PSI)
    surface_type: [SurfaceType; 4],     // Driving surface, see appendices
}

/// Telemetry for all the cars in the race.
///
/// It details various values that would be recorded on the car such as
/// speed, throttle application, DRS etc.
///
/// Note that the rev light configurations are presented
/// separately as well and will mimic real life driver preferences.
#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PacketCarTelemetryData {
    pub header: PacketHeader, // Header
    car_telemetry_data: [CarTelemetryData; 22],
    pub mfd_panel_index: u8, // Index of MFD panel open - 255 = MFD closed
    // Single player, race – 0 = Car setup, 1 = Pits
    // 2 = Damage, 3 =  Engine, 4 = Temperatures
    // May vary depending on game mode
    pub mfd_panel_index_secondary_player: u8, // See above
    pub suggested_gear: i8,                   // Suggested gear for the player (1-8)
                                              // 0 if no gear suggested
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct CarStatusData {
    traction_control: u8,         // Traction control - 0 = off, 1 = medium, 2 = full
    anti_lock_brakes: u8,         // 0 (off) - 1 (on)
    fuel_mix: u8,                 // Fuel mix - 0 = lean, 1 = standard, 2 = rich, 3 = max
    front_brake_bias: u8,         // Front brake bias (percentage)
    pit_limiter_status: u8,       // Pit limiter status - 0 = off, 1 = on
    fuel_in_tank: f32,            // Current fuel mass
    fuel_capacity: f32,           // Fuel capacity
    fuel_remaining_laps: f32,     // Fuel remaining in terms of laps (value on MFD)
    max_rpm: u16,                 // Cars max RPM, point of rev limiter
    idle_rpm: u16,                // Cars idle RPM
    max_gears: u8,                // Maximum number of gears
    drs_allowed: u8,              // 0 = not allowed, 1 = allowed
    drs_activation_distance: u16, // 0 = DRS not available, non-zero - DRS will be available in [X] metres
    actual_tyre_compound: u8,     // F1 Modern - 16 = C5, 17 = C4, 18 = C3, 19 = C2, 20 = C1
    // 7 = inter, 8 = wet
    // F1 Classic - 9 = dry, 10 = wet
    // F2 – 11 = super soft, 12 = soft, 13 = medium, 14 = hard
    // 15 = wet
    visual_tyre_compound: u8, // F1 visual (can be different from actual compound)
    // 16 = soft, 17 = medium, 18 = hard, 7 = inter, 8 = wet
    // F1 Classic – same as above
    // F2 ‘19, 15 = wet, 19 – super soft, 20 = soft
    // 21 = medium , 22 = hard
    tyres_age_laps: u8,    // Age in laps of the current set of tyres
    vehicle_fia_flags: i8, // -1 = invalid/unknown, 0 = none, 1 = green
    // 2 = blue, 3 = yellow, 4 = red
    ers_store_energy: f32, // ERS energy store in Joules
    ers_deploy_mode: u8,   // ERS deployment mode, 0 = none, 1 = medium
    // 2 = hotlap, 3 = overtake
    ers_harvested_this_lap_mguk: f32, // ERS energy harvested this lap by MGU-K
    ers_harvested_this_lap_mguh: f32, // ERS energy harvested this lap by MGU-H
    ers_deployed_this_lap: f32,       // ERS energy deployed this lap
    network_paused: u8,               // Whether the car is paused in a network game
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PacketCarStatusData {
    pub header: PacketHeader, // Header
    car_status_data: [CarStatusData; 22],
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct FinalClassificationData {
    position: u8,      // Finishing position
    num_laps: u8,      // Number of laps completed
    grid_position: u8, // Grid position of the car
    points: u8,        // Number of points scored
    num_pit_stops: u8, // Number of pit stops made
    result_status: u8, // Result status - 0 = invalid, 1 = inactive, 2 = active
    // 3 = finished, 4 = didnotfinish, 5 = disqualified
    // 6 = not classified, 7 = retired
    best_lap_time_in_ms: u32, // Best lap time of the session in milliseconds
    total_race_time: f64,     // Total race time in seconds without penalties
    penalties_time: u8,       // Total penalties accumulated in seconds
    num_penalties: u8,        // Number of penalties applied to this driver
    num_tyre_stints: u8,      // Number of tyres stints up to maximum
    tyre_stints_actual: [u8; 8], // Actual tyres used by this driver
    tyre_stints_visual: [u8; 8], // Visual tyres used by this driver
    tyre_stints_end_laps: [u8; 8], // The lap number stints end on
}

/// This packet details the final classification at the end of the race,
/// and the data will match with the post race results screen.
#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PacketFinalClassificationData {
    pub header: PacketHeader, // Header
    pub num_cars: u8,         // Number of cars in the final classification
    classification_data: [FinalClassificationData; 22],
}

#[repr(C, packed)]
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct LobbyInfoData {
    ai_controlled: u8,          // Whether the vehicle is AI (1) or Human (0) controlled
    team_id: TeamId,            // Team id - see appendix (255 if no team currently selected)
    nationality: NationalityId, // Nationality of the driver

    #[serde_as(as = "[_; 48]")]
    name: [u8; 48], // Name of participant in UTF-8 format – null terminated Truncated with ... (U+2026) if too long
    car_number: u8,   // Car number of the player
    ready_status: u8, // 0 = not ready, 1 = ready, 2 = spectating
}

/// This packet details the players currently in a multiplayer lobby.
/// It details each player’s selected car,
/// any AI involved in the game and also the ready status of each of the participants.
#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PacketLobbyInfoData {
    pub header: PacketHeader,

    // Packet specific data
    pub num_players: u8, // Number of players in the lobby data
    lobby_players: [LobbyInfoData; 22],
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct CarDamageData {
    tyres_wear: [f32; 4],        // Tyre wear (percentage)
    tyres_damage: [u8; 4],       // Tyre damage (percentage)
    brakes_damage: [u8; 4],      // Brakes damage (percentage)
    front_left_wing_damage: u8,  // Front left wing damage (percentage)
    front_right_wing_damage: u8, // Front right wing damage (percentage)
    rear_wing_damage: u8,        // Rear wing damage (percentage)
    floor_damage: u8,            // Floor damage (percentage)
    diffuser_damage: u8,         // Diffuser damage (percentage)
    sidepod_damage: u8,          // Sidepod damage (percentage)
    drs_fault: u8,               // Indicator for DRS fault, 0 = OK, 1 = fault
    ers_fault: u8,               // Indicator for ERS fault, 0 = OK, 1 = fault
    gear_box_damage: u8,         // Gear box damage (percentage)
    engine_damage: u8,           // Engine damage (percentage)
    engine_mguh_wear: u8,        // Engine wear MGU-H (percentage)
    engine_es_wear: u8,          // Engine wear ES (percentage)
    engine_ce_wear: u8,          // Engine wear CE (percentage)
    engine_ice_wear: u8,         // Engine wear ICE (percentage)
    engine_mguk_wear: u8,        // Engine wear MGU-K (percentage)
    engine_tc_wear: u8,          // Engine wear TC (percentage)
    engine_blown: u8,            // Engine blown, 0 = OK, 1 = fault
    engine_seized: u8,           // Engine seized, 0 = OK, 1 = fault
}

/// This packet details car damage parameters for all the cars in the race.
#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PacketCarDamageData {
    pub header: PacketHeader,
    car_damage_data: [CarDamageData; 22],
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct LapHistoryData {
    lap_time_in_ms: u32,     // Lap time in milliseconds
    sector1_time_in_ms: u16, // Sector 1 time in milliseconds
    sector2_time_in_ms: u16, // Sector 2 time in milliseconds
    sector3_time_in_ms: u16, // Sector 3 time in milliseconds
    lap_valid_bit_flags: u8, // 0x01 bit set-lap valid, 0x02 bit set-sector 1 valid
                             // 0x04 bit set-sector 2 valid, 0x08 bit set-sector 3 valid
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct TyreStintHistoryData {
    end_lap: u8,              // Lap the tyre usage ends on (255 of current tyre)
    tyre_actual_compound: u8, // Actual tyres used by this driver
    tyre_visual_compound: u8, // Visual tyres used by this driver
}

/// This packet contains lap times and tyre usage for the session.
///
/// This packet works slightly differently to other packets.
/// To reduce CPU and bandwidth, each packet relates to a specific vehicle and is sent every 1/20 s,
/// and the vehicle being sent is cycled through.
///
/// Therefore in a 20 car race you should receive an update for each vehicle at least once per second.
#[repr(C, packed)]
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PacketSessionHistoryData {
    pub header: PacketHeader, // Header

    pub car_idx: u8,         // Index of the car this lap data relates to
    pub num_laps: u8,        // Num laps in the data (including current partial lap)
    pub num_tyre_stints: u8, // Number of tyre stints in the data

    pub best_lap_time_lap_num: u8, // Lap the best lap time was achieved on
    pub best_sector1_lap_num: u8,  // Lap the best Sector 1 time was achieved on
    pub best_sector2_lap_num: u8,  // Lap the best Sector 2 time was achieved on
    pub best_sector3_lap_num: u8,  // Lap the best Sector 3 time was achieved on

    #[serde_as(as = "[_; 100]")]
    lap_history_data: [LapHistoryData; 100], // 100 laps of data max
    tyre_stints_history_data: [TyreStintHistoryData; 8],
}

/// A generic enum representing all possible F1 telemetry packet types
#[derive(Clone, Copy)]
pub enum TelemetryPacket {
    Motion(PacketMotionData),
    Session(PacketSessionData),
    LapData(PacketLapData),
    Event(PacketEventData),
    Participants(PacketParticipantsData),
    CarSetups(PacketCarSetupData),
    CarTelemetry(PacketCarTelemetryData),
    CarStatus(PacketCarStatusData),
    FinalClassification(PacketFinalClassificationData),
    LobbyInfo(PacketLobbyInfoData),
    CarDamage(PacketCarDamageData),
    SessionHistory(PacketSessionHistoryData),
}

impl TelemetryPacket {
    /// View the PacketHeader portion of the packet
    pub fn header(&self) -> &PacketHeader {
        match self {
            TelemetryPacket::Motion(packet) => &packet.header,
            TelemetryPacket::Session(packet) => &packet.header,
            TelemetryPacket::LapData(packet) => &packet.header,
            TelemetryPacket::Event(packet) => &packet.header,
            TelemetryPacket::Participants(packet) => &packet.header,
            TelemetryPacket::CarSetups(packet) => &packet.header,
            TelemetryPacket::CarTelemetry(packet) => &packet.header,
            TelemetryPacket::CarStatus(packet) => &packet.header,
            TelemetryPacket::FinalClassification(packet) => &packet.header,
            TelemetryPacket::LobbyInfo(packet) => &packet.header,
            TelemetryPacket::CarDamage(packet) => &packet.header,
            TelemetryPacket::SessionHistory(packet) => &packet.header,
        }
    }

    /// Get the name of the underlying packet type
    pub fn name(&self) -> &'static str {
        match self {
            TelemetryPacket::Motion(_) => "Motion",
            TelemetryPacket::Session(_) => "Session",
            TelemetryPacket::LapData(_) => "Lap Data",
            TelemetryPacket::Event(_) => "Event",
            TelemetryPacket::Participants(_) => "Participants",
            TelemetryPacket::CarSetups(_) => "Car Setups",
            TelemetryPacket::CarTelemetry(_) => "Car Telemetry",
            TelemetryPacket::CarStatus(_) => "Car Status",
            TelemetryPacket::FinalClassification(_) => "Final Classification",
            TelemetryPacket::LobbyInfo(_) => "Lobby Info",
            TelemetryPacket::CarDamage(_) => "Car Damage",
            TelemetryPacket::SessionHistory(_) => "Session History",
        }
    }

    /// Get the type of the underlying packet.
    ///
    /// For valid packet ids/type see [`packet_id`](PacketHeader::packet_id)
    pub fn packet_id(&self) -> PacketType {
        self.header().packet_id
    }

    /// Get the F1 CodeMasters format of the packet
    ///
    /// For example: 2022, 2021, 2020
    pub fn format(&self) -> u16 {
        self.header().packet_format
    }

    /// Get the unique identifier for the F1 session
    pub fn session_uid(&self) -> u64 {
        self.header().session_uid
    }

    /// Get the timestamp for the F1 session
    pub fn session_time(&self) -> f32 {
        self.header().session_time
    }

    /// Get the version of the underlying packet type, all start from 1
    pub fn version(&self) -> u8 {
        self.header().packet_version
    }

    pub fn as_motion(&self) -> Option<&PacketMotionData> {
        if let TelemetryPacket::Motion(p) = self {
            Some(p)
        } else {
            None
        }
    }

    pub fn as_session(&self) -> Option<&PacketSessionData> {
        if let TelemetryPacket::Session(p) = self {
            Some(p)
        } else {
            None
        }
    }

    pub fn as_lap_data(&self) -> Option<&PacketLapData> {
        if let TelemetryPacket::LapData(p) = self {
            Some(p)
        } else {
            None
        }
    }

    pub fn as_event(&self) -> Option<&PacketEventData> {
        if let TelemetryPacket::Event(p) = self {
            Some(p)
        } else {
            None
        }
    }

    pub fn as_participants(&self) -> Option<&PacketParticipantsData> {
        if let TelemetryPacket::Participants(p) = self {
            Some(p)
        } else {
            None
        }
    }

    pub fn as_car_setups(&self) -> Option<&PacketCarSetupData> {
        if let TelemetryPacket::CarSetups(p) = self {
            Some(p)
        } else {
            None
        }
    }

    pub fn as_car_telemetry(&self) -> Option<&PacketCarTelemetryData> {
        if let TelemetryPacket::CarTelemetry(p) = self {
            Some(p)
        } else {
            None
        }
    }

    pub fn as_car_status(&self) -> Option<&PacketCarStatusData> {
        if let TelemetryPacket::CarStatus(p) = self {
            Some(p)
        } else {
            None
        }
    }

    pub fn as_final_classification(&self) -> Option<&PacketFinalClassificationData> {
        if let TelemetryPacket::FinalClassification(p) = self {
            Some(p)
        } else {
            None
        }
    }

    pub fn as_lobby_info(&self) -> Option<&PacketLobbyInfoData> {
        if let TelemetryPacket::LobbyInfo(p) = self {
            Some(p)
        } else {
            None
        }
    }

    pub fn as_car_damage(&self) -> Option<&PacketCarDamageData> {
        if let TelemetryPacket::CarDamage(p) = self {
            Some(p)
        } else {
            None
        }
    }

    pub fn as_session_history(&self) -> Option<&PacketSessionHistoryData> {
        if let TelemetryPacket::SessionHistory(p) = self {
            Some(p)
        } else {
            None
        }
    }
}
