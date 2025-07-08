use crate::core::{ids::PacketType, PacketHeader, TelemetryPacket};

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
        PacketType::Motion => {
            TelemetryPacket::Motion(unsafe { std::ptr::read(buffer.as_ptr() as *const _) })
        }
        PacketType::Session => {
            TelemetryPacket::Session(unsafe { std::ptr::read(buffer.as_ptr() as *const _) })
        }
        PacketType::LapData => {
            TelemetryPacket::LapData(unsafe { std::ptr::read(buffer.as_ptr() as *const _) })
        }
        PacketType::Event => {
            TelemetryPacket::Event(unsafe { std::ptr::read(buffer.as_ptr() as *const _) })
        }
        PacketType::Participants => {
            TelemetryPacket::Participants(unsafe { std::ptr::read(buffer.as_ptr() as *const _) })
        }
        PacketType::CarSetups => {
            TelemetryPacket::CarSetups(unsafe { std::ptr::read(buffer.as_ptr() as *const _) })
        }
        PacketType::CarTelemetry => {
            TelemetryPacket::CarTelemetry(unsafe { std::ptr::read(buffer.as_ptr() as *const _) })
        }
        PacketType::CarStatus => {
            TelemetryPacket::CarStatus(unsafe { std::ptr::read(buffer.as_ptr() as *const _) })
        }
        PacketType::FinalClassification => TelemetryPacket::FinalClassification(unsafe {
            std::ptr::read(buffer.as_ptr() as *const _)
        }),
        PacketType::LobbyInfo => {
            TelemetryPacket::LobbyInfo(unsafe { std::ptr::read(buffer.as_ptr() as *const _) })
        }
        PacketType::CarDamage => {
            TelemetryPacket::CarDamage(unsafe { std::ptr::read(buffer.as_ptr() as *const _) })
        }
        PacketType::SessionHistory => {
            TelemetryPacket::SessionHistory(unsafe { std::ptr::read(buffer.as_ptr() as *const _) })
        }
        _ => return None,
    };

    Some(packet)
}
