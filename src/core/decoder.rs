use crate::core::{
    PacketCarDamageData, PacketCarSetupData, PacketCarStatusData, PacketCarTelemetryData,
    PacketEventData, PacketFinalClassificationData, PacketHeader, PacketLapData,
    PacketLobbyInfoData, PacketMotionData, PacketParticipantsData, PacketSessionData,
    PacketSessionHistoryData,
};

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

/// Attempt to create a TelemetryPacket from a buffer
///
/// Inspects the PacketHeader's packet_id field to create the respective
/// Packet structure. If the length of the buffer is less than the size of PacketHeader, or
/// packet_id is invalid, None will be returned.
///
/// See [`packet_id`](PacketHeader::packet_id) for a list of valid packet ID's.
///
/// # Safety
/// This function casts the buffer to a 'PacketHeader' struct using
/// std::ptr::read(). Based on the newly casted PacketHeader's packet_id field,
/// buffer will be casted to the respective packet struct, again using std::ptr::read().
///
///
/// The caller is responsible for ensuring the buffer is the correct
/// size and layout.
pub fn parse_packet(buffer: &[u8]) -> Option<TelemetryPacket> {
    if buffer.len() < std::mem::size_of::<PacketHeader>() {
        return None;
    }

    // Cast buffer to header to read the packet_id
    let header = unsafe { std::ptr::read(buffer.as_ptr() as *const PacketHeader) };

    // Based on packet id, cast to a specific structure for
    // respective, relevant data
    let packet = match header.packet_id {
        0 => TelemetryPacket::Motion(unsafe { std::ptr::read(buffer.as_ptr() as *const _) }),
        1 => TelemetryPacket::Session(unsafe { std::ptr::read(buffer.as_ptr() as *const _) }),
        2 => TelemetryPacket::LapData(unsafe { std::ptr::read(buffer.as_ptr() as *const _) }),
        3 => TelemetryPacket::Event(unsafe { std::ptr::read(buffer.as_ptr() as *const _) }),
        4 => TelemetryPacket::Participants(unsafe { std::ptr::read(buffer.as_ptr() as *const _) }),
        5 => TelemetryPacket::CarSetups(unsafe { std::ptr::read(buffer.as_ptr() as *const _) }),
        6 => TelemetryPacket::CarTelemetry(unsafe { std::ptr::read(buffer.as_ptr() as *const _) }),
        7 => TelemetryPacket::CarStatus(unsafe { std::ptr::read(buffer.as_ptr() as *const _) }),
        8 => TelemetryPacket::FinalClassification(unsafe {
            std::ptr::read(buffer.as_ptr() as *const _)
        }),
        9 => TelemetryPacket::LobbyInfo(unsafe { std::ptr::read(buffer.as_ptr() as *const _) }),
        10 => TelemetryPacket::CarDamage(unsafe { std::ptr::read(buffer.as_ptr() as *const _) }),
        11 => {
            TelemetryPacket::SessionHistory(unsafe { std::ptr::read(buffer.as_ptr() as *const _) })
        }
        _ => return None,
    };

    Some(packet)
}
