/// F1 22 ID and Type enum Definitions
/// Provided by CodeMaster
use serde::{Deserialize, Serialize};

/// Enum representing the packet type
/// from `packet_id` field in the PacketHeader struct
#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum PacketType {
    Motion = 0,
    Session = 1,
    LapData = 2,
    Event = 3,
    Participants = 4,
    CarSetups = 5,
    CarTelemetry = 6,
    CarStatus = 7,
    FinalClassification = 8,
    LobbyInfo = 9,
    CarDamage = 10,
    SessionHistory = 11,

    #[default]
    None = 255,
}

impl PacketType {
    /// Convert the enum type to the corresponding
    /// u8 as defined in CodeMasters PacketHeader documentation
    pub fn as_u8(&self) -> u8 {
        match self {
            PacketType::Motion => 0,
            PacketType::Session => 1,
            PacketType::LapData => 2,
            PacketType::Event => 3,
            PacketType::Participants => 4,
            PacketType::CarSetups => 5,
            PacketType::CarTelemetry => 6,
            PacketType::CarStatus => 7,
            PacketType::FinalClassification => 8,
            PacketType::LobbyInfo => 9,
            PacketType::CarDamage => 10,
            PacketType::SessionHistory => 11,
            PacketType::None => 255,
        }
    }
}

/// Enum representing a team based on a u8
/// from `team_id` field in a packet
#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub enum TeamId {
    Mercedes = 0,
    Ferrari = 1,
    RedBullRacing = 2,
    Williams = 3,
    AstonMartin = 4,
    Alpine = 5,
    AlphaTauri = 6,
    Haas = 7,
    McLaren = 8,
    AlfaRomeo = 9,
    Mercedes2020 = 85,
    Ferrari2020 = 86,
    RedBull2020 = 87,
    Williams2020 = 88,
    RacingPoint2020 = 89,
    Renault2020 = 90,
    AlphaTauri2020 = 91,
    Haas2020 = 92,
    McLaren2020 = 93,
    AlfaRomeo2020 = 94,
    AstonMartinDB11V12 = 95,
    AstonMartinVantageF1Edition = 96,
    AstonMartinVantageSafetyCar = 97,
    FerrariF8Tributo = 98,
    FerrariRoma = 99,
    McLaren720S = 100,
    McLarenArtura = 101,
    MercedesAMGGTBlackSeriesSafetyCar = 102,
    MercedesAMGGTRPro = 103,
    F1CustomTeam = 104,
    Prema21 = 106,
    UniVirtuosi21 = 107,
    Carlin21 = 108,
    Hitech21 = 109,
    ArtGP21 = 110,
    MPMotorsport21 = 111,
    Charouz21 = 112,
    Dams21 = 113,
    Campos21 = 114,
    BWT21 = 115,
    Trident21 = 116,
    MercedesAMGGTBlackSeries = 117,
    Prema22 = 118,
    Virtuosi22 = 119,
    Carlin22 = 120,
    Hitech22 = 121,
    ArtGP22 = 122,
    MPMotorsport22 = 123,
    Charouz22 = 124,
    Dams22 = 125,
    Campos22 = 126,
    VanAmersfoortRacing22 = 127,
    Trident22 = 128,

    #[default]
    None = 255,
}

/// Enum representing a driver based on
/// the `driver_id` field in a packet
#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub enum DriverId {
    CarlosSainz = 0,
    DaniilKvyat = 1,
    DanielRicciardo = 2,
    FernandoAlonso = 3,
    FelipeMassa = 4,
    KimiRaikkonen = 6,
    LewisHamilton = 7,
    MaxVerstappen = 9,
    NicoHulkenburg = 10,
    KevinMagnussen = 11,
    RomainGrosjean = 12,
    SebastianVettel = 13,
    SergioPerez = 14,
    ValtteriBottas = 15,
    EstebanOcon = 17,
    LanceStroll = 19,
    ArronBarnes = 20,
    MartinGiles = 21,
    AlexMurray = 22,
    LucasRoth = 23,
    IgorCorreia = 24,
    SophieLevasseur = 25,
    JonasSchiffer = 26,
    AlainForest = 27,
    JayLetourneau = 28,
    EstoSaari = 29,
    YasarAtiyeh = 30,
    CallistoCalabresi = 31,
    NaotaIzum = 32,
    HowardClarke = 33,
    WilheimKaufmann = 34,
    MarieLaursen = 35,
    FlavioNieves = 36,
    PeterBelousov = 37,
    KlimekMichalski = 38,
    SantiagoMoreno = 39,
    BenjaminCoppens = 40,
    NoahVisser = 41,
    GertWaldmuller = 42,
    JulianQuesada = 43,
    DanielJones = 44,
    ArtemMarkelov = 45,
    TadasukeMakino = 46,
    SeanGelael = 47,
    NyckDeVries = 48,
    JackAitken = 49,
    GeorgeRussell = 50,
    MaximilianGunther = 51,
    NireiFukuzumi = 52,
    LucaGhiotto = 53,
    LandoNorris = 54,
    SergioSetteCamara = 55,
    LouisDeletraz = 56,
    AntonioFuoco = 57,
    CharlesLeclerc = 58,
    PierreGasly = 59,
    AlexanderAlbon = 62,
    NicholasLatifi = 63,
    DorianBoccolacci = 64,
    NikoKari = 65,
    RobertoMerhi = 66,
    ArjunMaini = 67,
    AlessioLorandi = 68,
    RubenMeijer = 69,
    RashidNair = 70,
    JackTremblay = 71,
    DevonButler = 72,
    LukasWeber = 73,
    AntonioGiovinazzi = 74,
    RobertKubica = 75,
    AlainProst = 76,
    AyrtonSenna = 77,
    NobuharuMatsushita = 78,
    NikitaMazepin = 79,
    GuanyaZhou = 80,
    MickSchumacher = 81,
    CallumIlott = 82,
    JuanManuelCorrea = 83,
    JordanKing = 84,
    MahaveerRaghunathan = 85,
    TatianaCalderon = 86,
    AnthoineHubert = 87,
    GuilianoAlesi = 88,
    RalphBoschung = 89,
    MichaelSchumacher = 90,
    DanTicktum = 91,
    MarcusArmstrong = 92,
    ChristianLundgaard = 93,
    YukiTsunoda = 94,
    JehanDaruvala = 95,
    GulhermeSamaia = 96,
    PedroPiquet = 97,
    FelipeDrugovich = 98,
    RobertSchwartzman = 99,
    RoyNissany = 100,
    MarinoSato = 101,
    AidanJackson = 102,
    CasperAkkerman = 103,
    JensonButton = 109,
    DavidCoulthard = 110,
    NicoRosberg = 111,
    OscarPiastri = 112,
    LiamLawson = 113,
    JuriVips = 114,
    TheoPourchaire = 115,
    RichardVerschoor = 116,
    LirimZendeli = 117,
    DavidBeckmann = 118,
    AlessioDeledda = 121,
    BentViscaal = 122,
    EnzoFittipaldi = 123,
    MarkWebber = 125,
    JacquesVilleneuve = 126,
    JakeHughes = 127,
    FrederikVesti = 128,
    OlliCaldwell = 129,
    LoganSargeant = 130,
    CemBolukbasi = 131,
    AyumaIwasa = 132,
    ClementNovolak = 133,
    DennisHauger = 134,
    CalanWilliams = 135,
    JackDoohan = 136,
    AmauryCordeel = 137,
    MikaHakkinen = 138,

    #[default]
    NetworkHuman = 255,
}

/// Enum representing an F1 track based on
/// the `track_id` field in a packet
#[repr(i8)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub enum TrackId {
    #[default]
    Unknown = -1,

    Melbourne = 0,
    PaulRicard = 1,
    Shanghai = 2,
    SakhirBahrain = 3,
    Catalunya = 4,
    Monaco = 5,
    Montreal = 6,
    Silverstone = 7,
    Hockenheim = 8,
    Hungaroring = 9,
    Spa = 10,
    Monza = 11,
    Singapore = 12,
    Suzuka = 13,
    AbuDhabi = 14,
    Texas = 15,
    Brazil = 16,
    Austria = 17,
    Sochi = 18,
    Mexico = 19,
    BakuAzerbaijan = 20,
    SakhirShort = 21,
    SilverstoneShort = 22,
    TexasShort = 23,
    SuzukaShort = 24,
    Hanoi = 25,
    Zandvoort = 26,
    Imola = 27,
    Portimao = 28,
    Jeddah = 29,
    Miami = 30,
}

/// Enum representing a nationality based on
/// the `nationality` field in a packet
#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub enum NationalityId {
    American = 1,
    Argentinean = 2,
    Australian = 3,
    Austrian = 4,
    Azerbaijani = 5,
    Bahraini = 6,
    Belgian = 7,
    Bolivian = 8,
    Brazilian = 9,
    British = 10,
    Bulgarian = 11,
    Cameroonian = 12,
    Canadian = 13,
    Chilean = 14,
    Chinese = 15,
    Colombian = 16,
    CostaRican = 17,
    Croatian = 18,
    Cypriot = 19,
    Czech = 20,
    Danish = 21,
    Dutch = 22,
    Ecuadorian = 23,
    English = 24,
    Emirian = 25,
    Estonian = 26,
    Finnish = 27,
    French = 28,
    German = 29,
    Ghanaian = 30,
    Greek = 31,
    Guatemalan = 32,
    Honduran = 33,
    HongKonger = 34,
    Hungarian = 35,
    Icelander = 36,
    Indian = 37,
    Indonesian = 38,
    Irish = 39,
    Israeli = 40,
    Italian = 41,
    Jamaican = 42,
    Japanese = 43,
    Jordanian = 44,
    Kuwaiti = 45,
    Latvian = 46,
    Lebanese = 47,
    Lithuanian = 48,
    Luxembourger = 49,
    Malaysian = 50,
    Maltese = 51,
    Mexican = 52,
    Monegasque = 53,
    NewZealander = 54,
    Nicaraguan = 55,
    NorthernIrish = 56,
    Norwegian = 57,
    Omani = 58,
    Pakistani = 59,
    Panamanian = 60,
    Paraguayan = 61,
    Peruvian = 62,
    Polish = 63,
    Portuguese = 64,
    Qatari = 65,
    Romanian = 66,
    Russian = 67,
    Salvadoran = 68,
    Saudi = 69,
    Scottish = 70,
    Serbian = 71,
    Singaporean = 72,
    Slovakian = 73,
    Slovenian = 74,
    SouthKorean = 75,
    SouthAfrican = 76,
    Spanish = 77,
    Swedish = 78,
    Swiss = 79,
    Thai = 80,
    Turkish = 81,
    Uruguayan = 82,
    Ukrainian = 83,
    Venezuelan = 84,
    Barbadian = 85,
    Welsh = 86,
    Vietnamese = 87,

    #[default]
    None = 255,
}

/// Enum representing a game mode based on
/// the `gameMode` field in a packet
#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum GameModeId {
    EventMode = 0,
    GrandPrix = 3,
    TimeTrial = 5,
    Splitscreen = 6,
    OnlineCustom = 7,
    OnlineLeague = 8,
    CareerInvitational = 11,
    ChampionshipInvitational = 12,
    Championship = 13,
    OnlineChampionship = 14,
    OnlineWeeklyEvent = 15,
    Career = 19,
    CareerOnline = 20,
    Benchmark = 127,
}

/// Enum representing a ruleset based on
/// the `rule_set` field in a packet
#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum RulesetId {
    PracticeAndQualifying = 0,
    Race = 1,
    TimeTrial = 2,
    TimeAttack = 4,
    CheckpointChallenge = 6,
    Autocross = 8,
    Drift = 9,
    AverageSpeedZone = 10,
    RivalDuel = 11,
}

/// Type of contact a wheel is experiencing
#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SurfaceType {
    Tarmac = 0,
    RumbleStrip = 1,
    Concrete = 2,
    Rock = 3,
    Gravel = 4,
    Mud = 5,
    Sand = 6,
    Grass = 7,
    Water = 8,
    Cobblestone = 9,
    Metal = 10,
    Ridged = 11,
}

/// Used in the telemetry packet to determine
/// if any buttons are being held on the controlling device.
///
/// If the value below logical ANDed with the button
/// status is set then the corresponding button is being held.
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ButtonFlag {
    CrossorA = 0x00000001,
    TriangleorY = 0x00000002,
    CircleorB = 0x00000004,
    SquareorX = 0x00000008,
    DpadLeft = 0x00000010,
    DpadRight = 0x00000020,
    DpadUp = 0x00000040,
    DpadDown = 0x00000080,
    OptionsorMenu = 0x00000100,
    L1orLB = 0x00000200,
    R1orRB = 0x00000400,
    L2orLT = 0x00000800,
    R2orRT = 0x00001000,
    LeftStickClick = 0x00002000,
    RightStickClick = 0x00004000,
    RightStickLeft = 0x00008000,
    RightStickRight = 0x00010000,
    RightStickUp = 0x00020000,
    RightStickDown = 0x00040000,
    Special = 0x00080000,
    UDPAction1 = 0x00100000,
    UDPAction2 = 0x00200000,
    UDPAction3 = 0x00400000,
    UDPAction4 = 0x00800000,
    UDPAction5 = 0x01000000,
    UDPAction6 = 0x02000000,
    UDPAction7 = 0x04000000,
    UDPAction8 = 0x08000000,
    UDPAction9 = 0x10000000,
    UDPAction10 = 0x20000000,
    UDPAction11 = 0x40000000,
    UDPAction12 = 0x80000000,
}

impl ButtonFlag {
    /// Represent a button flag as the corresponding
    /// button name. For example, ButtonFlag::SquareorX corresponds to
    /// the Square or X button on controller.
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonFlag::CrossorA => "Cross/A",
            ButtonFlag::TriangleorY => "Triangle/Y",
            ButtonFlag::CircleorB => "Circle/B",
            ButtonFlag::SquareorX => "Square/X",
            ButtonFlag::DpadLeft => "Dpad Left",
            ButtonFlag::DpadRight => "Dpad Right",
            ButtonFlag::DpadUp => "Dpad Up",
            ButtonFlag::DpadDown => "Dpad Down",
            ButtonFlag::OptionsorMenu => "Option/Menu",
            ButtonFlag::L1orLB => "L1/LB",
            ButtonFlag::R1orRB => "R1/RB",
            ButtonFlag::L2orLT => "L2/LT",
            ButtonFlag::R2orRT => "R2/RT",
            ButtonFlag::LeftStickClick => "Left Stick Click",
            ButtonFlag::RightStickClick => "Right Stick Click",
            ButtonFlag::RightStickLeft => "Right Stick Left",
            ButtonFlag::RightStickRight => "Right Stick Right",
            ButtonFlag::RightStickUp => "Right Stick Up",
            ButtonFlag::RightStickDown => "Right Stick Down",
            ButtonFlag::Special => "Special",
            _ => "UDP Action",
        }
    }
}

/// Enum representing a penalty based on
/// the `penalty_type` field in a packet
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PenaltyType {
    DriveThrough = 0,
    StopGo = 1,
    GridPenalty = 2,
    PenaltyReminder = 3,
    TimePenalty = 4,
    Warning = 5,
    Disqualified = 6,
    RemovedFromFormationLap = 7,
    ParkedTooLongTimer = 8,
    TyreRegulations = 9,
    ThisLapInvalidated = 10,
    ThisAndNextLapInvalidated = 11,
    ThisLapInvalidatedWithoutReason = 12,
    ThisAndNextLapInvalidatedWithoutReason = 13,
    ThisAndPreviousLapInvalidated = 14,
    ThisAndPreviousLapInvalidatedWithoutReason = 15,
    Retired = 16,
    BlackFlagTimer = 17,
}

impl PenaltyType {
    /// Represent a PenaltyType enum as a string
    /// that flows with the sentence "xxx has received a _"
    pub fn as_str(&self) -> &'static str {
        match self {
            PenaltyType::DriveThrough => "Drive Through",
            PenaltyType::StopGo => "Stop Go Penalty",
            PenaltyType::GridPenalty => "Grid Penalty",
            PenaltyType::PenaltyReminder => "Penalty Reminder",
            PenaltyType::TimePenalty => "Time Penalty",
            PenaltyType::Warning => "Warning",
            PenaltyType::Disqualified => "Disqualification Penalty",
            PenaltyType::RemovedFromFormationLap => "Formation Lap Removal",
            PenaltyType::ParkedTooLongTimer => "Parked Too Long Penalty",
            PenaltyType::TyreRegulations => "Tyre Regulation Penalty",
            PenaltyType::ThisLapInvalidated => "Invalidated Lap (This Lap)",
            PenaltyType::ThisAndNextLapInvalidated => "Invalidated Lap (This And Next Lap)",
            PenaltyType::ThisLapInvalidatedWithoutReason => {
                "Invalidated Lap Without Reason (This Lap)"
            }
            PenaltyType::ThisAndNextLapInvalidatedWithoutReason => {
                "Invalidated Lap Without Reason (This And Next Lap)"
            }
            PenaltyType::ThisAndPreviousLapInvalidated => "Invalidated Lap (This And Previous Lap)",
            PenaltyType::ThisAndPreviousLapInvalidatedWithoutReason => {
                "Invalidated Lap Without Reason (This And Previous Lap)"
            }
            PenaltyType::Retired => "Retired",
            PenaltyType::BlackFlagTimer => "Black Flag Penalty",
        }
    }
}

/// Enum representing an infringement type based on
/// the `infringement_type` field in a packet
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum InfringementType {
    BlockingBySlowDriving = 0,
    BlockingByWrongWayDriving = 1,
    ReversingOffTheStartLine = 2,
    BigCollision = 3,
    SmallCollision = 4,
    CollisionFailedToHandBackPositionSingle = 5,
    CollisionFailedToHandBackPositionMultiple = 6,
    CornerCuttingGainedTime = 7,
    CornerCuttingOvertakeSingle = 8,
    CornerCuttingOvertakeMultiple = 9,
    CrossedPitExitLane = 10,
    IgnoringBlueFlags = 11,
    IgnoringYellowFlags = 12,
    IgnoringDriveThrough = 13,
    TooManyDriveThroughs = 14,
    DriveThroughReminderServeWithinNLaps = 15,
    DriveThroughReminderServeThisLap = 16,
    PitLaneSpeeding = 17,
    ParkedForTooLong = 18,
    IgnoringTyreRegulations = 19,
    TooManyPenalties = 20,
    MultipleWarnings = 21,
    ApproachingDisqualification = 22,
    TyreRegulationsSelectSingle = 23,
    TyreRegulationsSelectMultiple = 24,
    LapInvalidatedCornerCutting = 25,
    LapInvalidatedRunningWide = 26,
    CornerCuttingRanWideGainedTimeMinor = 27,
    CornerCuttingRanWideGainedTimeSignificant = 28,
    CornerCuttingRanWideGainedTimeExtreme = 29,
    LapInvalidatedWallRiding = 30,
    LapInvalidatedFlashbackUsed = 31,
    LapInvalidatedResetToTrack = 32,
    BlockingThePitlane = 33,
    JumpStart = 34,
    SafetyCarToCarCollision = 35,
    SafetyCarIllegalOvertake = 36,
    SafetyCarExceedingAllowedPace = 37,
    VirtualSafetyCarExceedingAllowedPace = 38,
    FormationLapBelowAllowedSpeed = 39,
    FormationLapParking = 40,
    RetiredMechanicalFailure = 41,
    RetiredTerminallyDamaged = 42,
    SafetyCarFallingTooFarBack = 43,
    BlackFlagTimer = 44,
    UnservedStopGoPenalty = 45,
    UnservedDriveThroughPenalty = 46,
    EngineComponentChange = 47,
    GearboxChange = 48,
    ParcFermeChange = 49,
    LeagueGridPenalty = 50,
    RetryPenalty = 51,
    IllegalTimeGain = 52,
    MandatoryPitstop = 53,
    AttributeAssigned = 54,
}

/// Enum representing a session type based on
/// the `session_type` field in a packet
#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub enum SessionType {
    #[default]
    Unknown = 0,

    P1 = 1,
    P2 = 2,
    P3 = 3,
    ShortP = 4,
    Q1 = 5,
    Q2 = 6,
    Q3 = 7,
    ShortQ = 8,
    OSQ = 9,
    R = 10,
    R2 = 11,
    R3 = 12,
    TimeTrial = 13,
}

/// Enum representing a state of weather based on
/// the `weather` field in a packet
#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub enum WeatherType {
    #[default]
    Clear = 0,

    LightCloud = 1,
    Overcast = 2,
    LightRain = 3,
    HeavyRain = 4,
    Storm = 5,
}

/// Enum representing the formula mode based on
/// the `formula` field in a packet. e.g: f1, f2, etc
#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub enum FormulaType {
    #[default]
    F1Modern = 0,

    F1Classic = 1,
    F2 = 2,
    F1Generic = 3,
    Beta = 4,
    Supercards = 5,
    Esports = 6,
    F22021 = 7,
}

/// Enum representing the length of a session based on
/// the `session_length` field in a packet
#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub enum SessionLength {
    #[default]
    None = 0,

    VeryShort = 2,
    Short = 3,
    Medium = 4,
    MediumLong = 5,
    Long = 6,
    Full = 7,
}

/// Enum representing an event based on
/// the `id` field in PacketEventData
#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventId {
    SessionStarted,
    SessionEnded,
    FastestLap,
    Retirement,
    DRSEnabled,
    DRSDisabled,
    TeamMateInPits,
    ChequeredFlag,
    RaceWinner,
    PenaltyIssued,
    SpeedTrapTriggered,
    StartLights,
    LightsOut,
    DriveThroughServed,
    StopGoServed,
    Flashback,
    ButtonStatus,
}
