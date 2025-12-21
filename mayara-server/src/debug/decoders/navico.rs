//! Navico protocol decoder.
//!
//! Decodes Navico binary UDP protocol messages (Simrad, B&G, Lowrance).

use super::ProtocolDecoder;
use crate::debug::{DecodedMessage, IoDirection};

// =============================================================================
// NavicoDecoder
// =============================================================================

/// Decoder for Navico radar protocol.
///
/// Navico uses binary UDP packets with various report types.
pub struct NavicoDecoder;

impl ProtocolDecoder for NavicoDecoder {
    fn decode(&self, data: &[u8], direction: IoDirection) -> DecodedMessage {
        if data.is_empty() {
            return DecodedMessage::Unknown {
                reason: "Empty data".to_string(),
                partial: None,
            };
        }

        // Navico packets typically have a report type in the first few bytes
        // The exact format varies by message type

        let message_type = identify_navico_message(data, direction);
        let (description, fields) = decode_navico_fields(data, &message_type);

        DecodedMessage::Navico {
            message_type,
            report_id: data.first().copied(),
            fields,
            description,
        }
    }

    fn brand(&self) -> &'static str {
        "navico"
    }
}

/// Identify the type of Navico message.
fn identify_navico_message(data: &[u8], direction: IoDirection) -> String {
    // Basic identification based on packet structure
    if data.len() >= 2 {
        let first_byte = data[0];

        // Spoke data typically starts with specific patterns
        if data.len() > 100 {
            return "spoke".to_string();
        }

        // Status/control reports
        match first_byte {
            0x01 => return "status".to_string(),
            0x02 => return "settings".to_string(),
            0x03 => return "firmware".to_string(),
            0x04 => return "diagnostic".to_string(),
            _ => {}
        }

        // Check for common patterns
        if direction == IoDirection::Send && data.len() < 20 {
            return "command".to_string();
        }
    }

    "unknown".to_string()
}

/// Decode Navico message fields.
fn decode_navico_fields(data: &[u8], message_type: &str) -> (Option<String>, serde_json::Value) {
    match message_type {
        "spoke" => {
            // Extract basic spoke info
            let angle = if data.len() >= 4 {
                u16::from_le_bytes([data[0], data[1]]) as i32
            } else {
                0
            };
            (
                Some(format!("Spoke data (angle: {})", angle)),
                serde_json::json!({
                    "angle": angle,
                    "length": data.len(),
                    "firstBytes": format!("{:02x?}", &data[..data.len().min(16)])
                }),
            )
        }
        "status" => (
            Some("Status report".to_string()),
            serde_json::json!({
                "length": data.len(),
                "firstBytes": format!("{:02x?}", &data[..data.len().min(32)])
            }),
        ),
        "command" => (
            Some("Control command".to_string()),
            serde_json::json!({
                "length": data.len(),
                "bytes": format!("{:02x?}", data)
            }),
        ),
        _ => (
            None,
            serde_json::json!({
                "length": data.len(),
                "firstBytes": format!("{:02x?}", &data[..data.len().min(32)])
            }),
        ),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_short_packet() {
        let decoder = NavicoDecoder;
        let msg = decoder.decode(&[0x01, 0x02, 0x03], IoDirection::Recv);

        match msg {
            DecodedMessage::Navico { message_type, .. } => {
                assert_eq!(message_type, "status");
            }
            _ => panic!("Expected Navico message"),
        }
    }

    #[test]
    fn test_decode_empty() {
        let decoder = NavicoDecoder;
        let msg = decoder.decode(&[], IoDirection::Recv);

        assert!(matches!(msg, DecodedMessage::Unknown { .. }));
    }
}
