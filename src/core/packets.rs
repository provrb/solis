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
    DriverId, FormulaType, InfringementType, NationalityId, PenaltyType, SessionLength,
    SurfaceType, TeamId,
};

/// Every packet will have the following header.
#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PacketHeader {
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
#[derive(Debug, Clone, Copy)]
pub struct PacketSessionData {
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

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
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
#[derive(Debug, Default, Clone, Copy)]
pub struct PacketLapData {
    header: PacketHeader,
    lap_data: [LapData; 22],      // Lap data for all cars on track
    time_trial_pb_car_idx: u8,    // Index of Personal Best car in time trial (255 if invalid)
    time_trial_rival_car_idx: u8, // Index of Rival car in time trial (255 if invalid)
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
struct FastestLap {
    vehicle_idx: u8, // Vehicle index of car achieving fastest lap
    lap_time: f32,   // Lap time is in seconds
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
struct Retirement {
    vehicle_idx: u8, // Vehicle index of car retiring
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
struct TeamMateInPits {
    vehicle_idx: u8, // Vehicle index of team mate
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
struct RaceWinner {
    vehicle_idx: u8, // Vehicle index of the race winner
}
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct Penalty {
    penalty_type: PenaltyType,           // Penalty type – see Appendices
    infringement_type: InfringementType, // Infringement type – see Appendices
    vehicle_idx: u8,                     // Vehicle index of the car the penalty is applied to
    other_vehicle_idx: u8,               // Vehicle index of the other car involved
    time: u8,                            // Time gained, or time spent doing action in seconds
    lap_num: u8,                         // Lap the penalty occurred on
    places_gained: u8,                   // Number of places gained by this
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
struct SpeedTrap {
    vehicle_idx: u8, // Vehicle index of the vehicle triggering speed trap
    speed: f32,      // Top speed achieved in kilometres per hour
    is_overall_fastest_in_session: u8, // Overall fastest speed in session = 1, otherwise 0
    is_driver_fastest_in_session: u8, // Fastest speed for driver in session = 1, otherwise 0
    fastest_vehicle_idx_in_session: u8, // Vehicle index of the vehicle that is the fastest in this session
    fastest_speed_in_session: f32, // Speed of the vehicle that is the fastest
                                   // in this session
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
struct StartLights {
    num_lights: u8, // Number of lights showing
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
struct DriveThroughPenaltyServed {
    vehicle_idx: u8, // Vehicle index of the vehicle serving drive through
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
struct StopGoPenaltyServed {
    vehicle_idx: u8, // Vehicle index of the vehicle serving stop go
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
struct Flashback {
    flashback_frame_identifier: u32, // Frame identifier flashed back to
    flashback_session_time: f32,     // Session time flashed back to
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
struct Buttons {
    button_status: u32, // Bit flags specifying which buttons are being pressed
                        // currently - see appendices
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
union EventDataDetails {
    fastest_lap: FastestLap,
    retirement: Retirement,
    teammate_in_pits: TeamMateInPits,
    race_winner: RaceWinner,
    penalty: Penalty,
    speed_trap: SpeedTrap,
    start_lights: StartLights,
    drive_through_penalty_served: DriveThroughPenaltyServed,
    stop_go_penalty_served: StopGoPenaltyServed,
    flashback: Flashback,
    buttons: Buttons,
}

/// This packet gives details of events that happen during the course of a session.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PacketEventData {
    header: PacketHeader,

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
    event_string_code: [u8; 4],

    event_details: EventDataDetails, // Event details - should be interpreted differently
                                     // for each type
}
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct ParticipantData {
    ai_controlled: u8,   // Whether the vehicle is AI (1) or Human (0) controlled
    driver_id: DriverId, // Driver id - see appendix, 255 if network human
    network_id: u8,      // Network id – unique identifier for network players
    team_id: TeamId,
    my_team: u8,                // My team flag – 1 = My Team, 0 = otherwise
    race_number: u8,            // Race number of the car
    nationality: NationalityId, // Nationality of the driver
    name: [char; 48], // Name of participant in UTF-8 format – null terminated. Truncated with … (U+2026) if too long
    your_telemetry: u8, // The player's UDP setting, 0 = restricted, 1 = public
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
#[derive(Debug, Clone, Copy)]
pub struct PacketParticipantsData {
    header: PacketHeader,
    num_active_cars: u8, // Number of active cars in the data – should match number of cars on HUD
    participants: [ParticipantData; 22],
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
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
#[derive(Debug, Default, Clone, Copy)]
pub struct PacketCarSetupData {
    header: PacketHeader,
    car_setups: [CarSetupData; 22],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct CarTelemetryData {
    speed: u16,                       // Speed of car in kilometres per hour
    throttle: f32,                    // Amount of throttle applied (0.0 to 1.0)
    steer: f32,                       // Steering (-1.0 (full lock left) to 1.0 (full lock right))
    brake: f32,                       // Amount of brake applied (0.0 to 1.0)
    clutch: u8,                       // Amount of clutch applied (0 to 100)
    gear: i8,                         // Gear selected (1-8, N=0, R=-1)
    engine_rpm: u16,                  // Engine RPM
    drs: u8,                          // 0 = off, 1 = on
    rev_lights_percent: u8,           // Rev lights indicator (percentage)
    rev_lights_bit_value: u16,        // Rev lights (bit 0 = leftmost LED, bit 14 = rightmost LED)
    brakes_temperature: [u16; 4],     // Brakes temperature (celsius)
    tyres_surface_temperature: [u8; 4], // Tyres surface temperature (celsius)
    tyres_inner_temperature: [u8; 4],   // Tyres inner temperature (celsius)
    engine_temperature: u16,          // Engine temperature (celsius)
    tyres_pressure: [f32; 4],         // Tyres pressure (PSI)
    surface_type: [SurfaceType; 4],   // Driving surface, see appendices
}

/// Telemetry for all the cars in the race. 
/// 
/// It details various values that would be recorded on the car such as 
/// speed, throttle application, DRS etc. 
/// 
/// Note that the rev light configurations are presented 
/// separately as well and will mimic real life driver preferences.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PacketCarTelemetryData {
    header: PacketHeader, // Header
    car_telemetry_data: [CarTelemetryData; 22],
    mfd_panel_index: u8, // Index of MFD panel open - 255 = MFD closed
    // Single player, race – 0 = Car setup, 1 = Pits
    // 2 = Damage, 3 =  Engine, 4 = Temperatures
    // May vary depending on game mode
    mfd_panel_index_secondary_player: u8, // See above
    suggested_gear: i8,                   // Suggested gear for the player (1-8)
                                          // 0 if no gear suggested
}
