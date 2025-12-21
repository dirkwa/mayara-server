//! Garmin protocol decoder.
//!
//! Decodes Garmin binary UDP protocol messages (xHD series).

use super::ProtocolDecoder;
use crate::debug::{DecodedMessage, IoDirection};

// =============================================================================
// GarminDecoder
// =============================================================================

/// Decoder for Garmin radar protocol.
///
/// Garmin uses a relatively simple binary UDP protocol:
/// - 12-byte command packets
/// - Multicast status broadcasts
/// - 1440 spokes per revolution
pub struct GarminDecoder;

impl ProtocolDecoder for GarminDecoder {
    fn decode(&self, data: &[u8], direction: IoDirection) -> DecodedMessage {
        if data.is_empty() {
            return DecodedMessage::Unknown {
                reason: "Empty data".to_string(),
                partial: None,
            };
        }

        let (message_type, description, fields) = decode_garmin(data, direction);

        DecodedMessage::Garmin {
            message_type,
            fields,
            description,
        }
    }

    fn brand(&self) -> &'static str {
        "garmin"
    }
}

/// Decode a Garmin packet.
fn decode_garmin(
    data: &[u8],
    direction: IoDirection,
) -> (String, Option<String>, serde_json::Value) {
    // Garmin commands are typically 12 bytes
    if direction == IoDirection::Send && data.len() == 12 {
        return decode_garmin_command(data);
    }

    // Status packets (received on multicast)
    if data.len() >= 8 && data.len() < 100 {
        return (
            "status".to_string(),
            Some("Status report".to_string()),
            serde_json::json!({
                "length": data.len(),
                "firstBytes": format!("{:02x?}", &data[..data.len().min(16)])
            }),
        );
    }

    // Spoke data (longer packets)
    if data.len() > 100 {
        // Try to extract angle from spoke
        let angle = if data.len() >= 4 {
            u16::from_le_bytes([data[0], data[1]]) as i32
        } else {
            0
        };

        return (
            "spoke".to_string(),
            Some(format!("Spoke data (angle: {})", angle)),
            serde_json::json!({
                "angle": angle,
                "length": data.len()
            }),
        );
    }

    // Unknown
    (
        "unknown".to_string(),
        None,
        serde_json::json!({
            "length": data.len(),
            "bytes": format!("{:02x?}", &data[..data.len().min(32)])
        }),
    )
}

/// Decode a 12-byte Garmin command.
fn decode_garmin_command(data: &[u8]) -> (String, Option<String>, serde_json::Value) {
    if data.len() != 12 {
        return (
            "command".to_string(),
            None,
            serde_json::json!({"length": data.len()}),
        );
    }

    // Command ID is typically in the first few bytes
    let cmd_type = data[0];
    let value = data.get(4).copied().unwrap_or(0);

    let desc = match cmd_type {
        0x01 => Some(format!(
            "Power: {}",
            if value == 1 { "ON" } else { "OFF" }
        )),
        0x02 => Some(format!("Range: {}", value)),
        0x03 => Some(format!("Gain: {}", value)),
        0x04 => Some(format!("Sea: {}", value)),
        0x05 => Some(format!("Rain: {}", value)),
        _ => None,
    };

    (
        "command".to_string(),
        desc,
        serde_json::json!({
            "commandType": format!("0x{:02x}", cmd_type),
            "value": value,
            "bytes": format!("{:02x?}", data)
        }),
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_command() {
        let decoder = GarminDecoder;
        let data = [0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let msg = decoder.decode(&data, IoDirection::Send);

        match msg {
            DecodedMessage::Garmin {
                message_type,
                description,
                ..
            } => {
                assert_eq!(message_type, "command");
                assert!(description.unwrap().contains("Power"));
            }
            _ => panic!("Expected Garmin message"),
        }
    }

    #[test]
    fn test_decode_empty() {
        let decoder = GarminDecoder;
        let msg = decoder.decode(&[], IoDirection::Recv);

        assert!(matches!(msg, DecodedMessage::Unknown { .. }));
    }
}
