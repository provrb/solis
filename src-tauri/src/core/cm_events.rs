/// Dictionary containing the defined CodeMaster F1
/// Events and Event structure definition.
///
/// This is not to be confused with frontend-backend events.
///
/// All CodeMasters events are listed below as such:
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
///
/// 'Code' is a 4 character code defined in CodeMasters' PacketEventData
/// to represent an event. See [`PacketEventData`](PacketEventData)
use std::collections::HashMap;
use std::sync::LazyLock;

use crate::core::ids::EventId;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Event {
    pub event: &'static str,
    pub code: &'static str,
    pub description: &'static str,
    pub id: EventId,
}

pub static CM_EVENTS: LazyLock<HashMap<&str, Event>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(
        "SSTA",
        Event {
            event: "Session Started",
            code: "SSTA",
            description: "Sent when the session starts",
            id: EventId::SessionStarted,
        },
    );
    m.insert(
        "SEND",
        Event {
            event: "Session Ended",
            code: "SEND",
            description: "Sent when the session ends",
            id: EventId::SessionEnded,
        },
    );
    m.insert(
        "FTLP",
        Event {
            event: "Fastest Lap",
            code: "FTLP",
            description: "When a driver achieves the fastest lap",
            id: EventId::FastestLap,
        },
    );
    m.insert(
        "RTMT",
        Event {
            event: "Retirement",
            code: "RTMT",
            description: "When a driver retires",
            id: EventId::Retirement,
        },
    );
    m.insert(
        "DRSE",
        Event {
            event: "DRS enabled",
            code: "DRSE",
            description: "Race control have enabled DRS",
            id: EventId::DRSEnabled,
        },
    );
    m.insert(
        "DRSD",
        Event {
            event: "DRS disabled",
            code: "DRSD",
            description: "Race control have disabled DRS",
            id: EventId::DRSDisabled,
        },
    );
    m.insert(
        "TMPT",
        Event {
            event: "Team mate in pits",
            code: "TMPT",
            description: "Your team mate has entered the pits",
            id: EventId::TeamMateInPits,
        },
    );
    m.insert(
        "CHQF",
        Event {
            event: "Chequered flag",
            code: "CHQF",
            description: "The chequered flag has been waved",
            id: EventId::ChequeredFlag,
        },
    );
    m.insert(
        "RCWN",
        Event {
            event: "Race Winner",
            code: "RCWN",
            description: "The race winner is announced",
            id: EventId::RaceWinner,
        },
    );
    m.insert(
        "PENA",
        Event {
            event: "Penalty Issued",
            code: "PENA",
            description: "A penalty has been issued – details in event",
            id: EventId::PenaltyIssued,
        },
    );
    m.insert(
        "SPTP",
        Event {
            event: "Speed Trap Triggered",
            code: "SPTP",
            description: "Speed trap has been triggered by fastest speed",
            id: EventId::SpeedTrapTriggered,
        },
    );
    m.insert(
        "STLG",
        Event {
            event: "Start lights",
            code: "STLG",
            description: "Start lights – number shown",
            id: EventId::StartLights,
        },
    );
    m.insert(
        "LGOT",
        Event {
            event: "Lights out",
            code: "LGOT",
            description: "Lights out",
            id: EventId::LightsOut,
        },
    );
    m.insert(
        "DTSV",
        Event {
            event: "Drive through served",
            code: "DTSV",
            description: "Drive through penalty served",
            id: EventId::DriveThroughServed,
        },
    );
    m.insert(
        "SGSV",
        Event {
            event: "Stop go served",
            code: "SGSV",
            description: "Stop go penalty served",
            id: EventId::StopGoServed,
        },
    );
    m.insert(
        "FLBK",
        Event {
            event: "Flashback",
            code: "FLBK",
            description: "Flashback activated",
            id: EventId::Flashback,
        },
    );
    m.insert(
        "BUTN",
        Event {
            event: "Button status",
            code: "BUTN",
            description: "Button status changed",
            id: EventId::ButtonStatus,
        },
    );
    m
});
