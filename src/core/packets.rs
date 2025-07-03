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
use crate::core::{
    ids::{SessionType, TrackId, WeatherType},
    FormulaType, SessionLength,
};

/// Every packet will have the following header.
#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
struct PacketHeader {
    packet_format: u16,
    game_major_version: u8,
    game_minor_version: u8,
    packet_version: u8,

    /// Identifier for the packet type, see below
    /// Packet Name          - Val - Description
    /// Motion               -  0  - Contains all motion data for player’s car – only sent while player is in control
    /// Session              -  1  - Data about the session – track, time left
    /// Lap Data             -  2  - Data about all the lap times of cars in the session
    /// Event                -  3  - Various notable events that happen during a session
    /// Participants         -  4  - List of participants in the session, mostly relevant for multiplayer
    /// Car Setups           -  5  - Packet detailing car setups for cars in the race
    /// Car Telemetry        -  6  - Telemetry data for all cars
    /// Car Status           -  7  - Status data for all cars
    /// Final Classification -  8  - Final classification confirmation at the end of a race
    /// Lobby Info           -  9  - Information about players in a multiplayer lobby
    /// Car Damage           -  10 - Damage status for all cars
    /// Session History      -  11 - Lap and tyre data for session
    packet_id: u8,
    session_uid: u64,
    session_time: f32,
    frame_identifier: u32,
    player_car_index: u8,

    /// Index of secondary player's car in the array (splitscreen)
    /// 255 if no second player
    secondary_player_car_index: u8,
}

/// Physics data for a vehicle
#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
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
    g_force_lateral: i16,      // Lateral G-Force component
    g_force_longitudinal: i16, // Longitudinal G-Force component
    g_force_vertical: i16,     // Vertical G-Force component
    yaw: i16,                  // Yaw angle in radians
    pitch: i16,                // Pitch angle in radians
    roll: i16,                 // Roll angle in radians
}

/// Physics data for all the cars being driven.
///
/// Includes additional data for the car being driven
/// with the goal of being able to drive a motion platform setup.
#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PacketMotionData {
    header: PacketHeader,                 // Header
    car_motion_data: [CarMotionData; 22], // Data for all cars on track

    // Extra player car ONLY data
    suspension_velocity: [f32; 4],     // RL, RR, FL, FR
    suspension_acceleration: [f32; 4], // RL, RR, FL, FR
    suspension_position: [f32; 4],     // RL, RR, FL, FR
    wheel_speed: [f32; 4],             // Speed of each wheel
    wheel_slip: [f32; 4],              // Slip ratio for each wheel
    local_velocity_x: f32,             // Velocity in local space
    local_velocity_y: f32,             // Velocity in local space
    local_velocity_z: f32,             // Velocity in local space
    angular_velocity_x: f32,           // Angular velocity x-component
    angular_velocity_y: f32,           // Angular velocity y-component
    angular_velocity_z: f32,           // Angular velocity z-component
    angular_acceleration_x: f32,       // Angular velocity x-component
    angular_acceleration_y: f32,       // Angular velocity y-component
    angular_acceleration_z: f32,       // Angular velocity z-component
    front_wheels_angle: f32,           // Current front wheels angle in radians
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
struct MarshalZone {
    zone_start: f32, // Fraction (0..1) of way through the lap the marshal zone starts
    zone_flag: i8,   // -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow, 4 = red
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
struct WeatherForecastSample {
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
#[derive(Debug, Clone)]
struct PacketSessionData {
    header: PacketHeader, // Header

    weather: WeatherType,
    track_temperature: u8, // Track temp. in degrees celsius
    air_temperature: u8,   // Air temp. in degrees celsius
    total_laps: u8,        // Total number of laps in this race
    track_length: u16,     // Track length in metres
    session_type: SessionType,
    track_id: TrackId,
    formula: FormulaType,
    session_time_left: u16,           // Time left in session in seconds
    session_duration: u16,            // Session duration in seconds
    pit_speed_limit: u8,              // Pit speed limit in kilometres per hour
    game_paused: u8,                  // Whether the game is paused – network game only
    is_spectating: u8,                // Whether the player is spectating
    spectator_car_index: u8,          // Index of the car being spectated
    sli_pro_native_support: u8,       // SLI Pro support, 0 = inactive, 1 = active
    num_marshal_zones: u8,            // Number of marshal zones to follow
    marshal_zones: [MarshalZone; 21], // List of marshal zones – max 21
    safety_car_status: u8,            // 0 = no safety car, 1 = full
    // 2 = virtual, 3 = formation lap
    network_game: u8,                                      // 0 = offline, 1 = online
    num_weather_forecast_samples: u8,                      // Number of weather samples to follow
    weather_forecast_samples: [WeatherForecastSample; 56], // Array of weather forecast samples
    forecast_accuracy: u8,                                 // 0 = Perfect, 1 = Approximate
    ai_difficulty: u8,                                     // AI Difficulty rating – 0-110
    season_link_identifier: u32, // Identifier for season - persists across saves
    weekend_link_identifier: u32, // Identifier for weekend - persists across saves
    session_link_identifier: u32, // Identifier for session - persists across saves
    pit_stop_window_ideal_lap: u8, // Ideal lap to pit on for current strategy (player)
    pit_stop_window_latest_lap: u8, // Latest lap to pit on for current strategy (player)
    pit_stop_rejoin_position: u8, // Predicted position to rejoin at (player)
    steering_assist: u8,         // 0 = off, 1 = on
    braking_assist: u8,          // 0 = off, 1 = low, 2 = medium, 3 = high
    gearbox_assist: u8,          // 1 = manual, 2 = manual & suggested gear, 3 = auto
    pit_assist: u8,              // 0 = off, 1 = on
    pit_release_assist: u8,      // 0 = off, 1 = on
    ers_assist: u8,              // 0 = off, 1 = on
    drs_assist: u8,              // 0 = off, 1 = on
    dynamic_racing_line: u8,     // 0 = off, 1 = corners only, 2 = full
    dynamic_racing_line_type: u8, // 0 = 2D, 1 = 3D
    game_mode: u8,               // Game mode id - see appendix
    rule_set: u8,                // Ruleset - see appendix
    time_of_day: u32,            // Local time of day - minutes since midnight
    session_length: SessionLength,
}
