//! Raymarine protocol decoder.
//!
//! Decodes Raymarine binary UDP protocol messages (Quantum, RD series).

use super::ProtocolDecoder;
use crate::debug::{DecodedMessage, IoDirection};

// =============================================================================
// RaymarineDecoder
// =============================================================================

/// Decoder for Raymarine radar protocol.
///
/// Raymarine has two main protocol variants:
/// - Quantum (solid-state): 2-byte opcode format
/// - RD (magnetron): Different format with lead bytes
pub struct RaymarineDecoder;

impl ProtocolDecoder for RaymarineDecoder {
    fn decode(&self, data: &[u8], direction: IoDirection) -> DecodedMessage {
        if data.is_empty() {
            return DecodedMessage::Unknown {
                reason: "Empty data".to_string(),
                partial: None,
            };
        }

        let (variant, message_type, description, fields) = decode_raymarine(data, direction);

        DecodedMessage::Raymarine {
            message_type,
            variant,
            fields,
            description,
        }
    }

    fn brand(&self) -> &'static str {
        "raymarine"
    }
}

/// Decode a Raymarine packet.
fn decode_raymarine(
    data: &[u8],
    _direction: IoDirection,
) -> (Option<String>, String, Option<String>, serde_json::Value) {
    // Try to identify variant and message type
    if data.len() >= 56 {
        // Could be a beacon packet (56 bytes)
        if is_beacon(data) {
            return (
                None,
                "beacon".to_string(),
                Some("Discovery beacon".to_string()),
                serde_json::json!({
                    "length": data.len(),
                    "firstBytes": format!("{:02x?}", &data[..data.len().min(32)])
                }),
            );
        }
    }

    // Quantum format: [opcode_lo, opcode_hi, 0x28, value, ...]
    if data.len() >= 4 && data.get(2) == Some(&0x28) {
        let opcode = u16::from_le_bytes([data[0], data[1]]);
        let value = data.get(3).copied().unwrap_or(0);
        let (desc, fields) = decode_quantum_command(opcode, value, &data[4..]);

        return (
            Some("quantum".to_string()),
            "command".to_string(),
            desc,
            fields,
        );
    }

    // RD format: [0x00, 0xc1, lead, value, 0x00, ...]
    if data.len() >= 5 && data.starts_with(&[0x00, 0xc1]) {
        let lead = data[2];
        let value = data[3];
        let (desc, fields) = decode_rd_command(lead, value);

        return (
            Some("rd".to_string()),
            "command".to_string(),
            desc,
            fields,
        );
    }

    // Spoke data (typically longer packets)
    if data.len() > 100 {
        return (
            None,
            "spoke".to_string(),
            Some("Spoke data".to_string()),
            serde_json::json!({
                "length": data.len()
            }),
        );
    }

    // Unknown
    (
        None,
        "unknown".to_string(),
        None,
        serde_json::json!({
            "length": data.len(),
            "firstBytes": format!("{:02x?}", &data[..data.len().min(32)])
        }),
    )
}

/// Check if data looks like a beacon packet.
fn is_beacon(data: &[u8]) -> bool {
    // Beacons are typically 56 bytes with specific patterns
    data.len() == 56 || data.len() == 36
}

/// Decode a Quantum format command.
fn decode_quantum_command(
    opcode: u16,
    value: u8,
    _rest: &[u8],
) -> (Option<String>, serde_json::Value) {
    let desc = match opcode {
        0xc401 => Some(format!("Gain: {}", value)),
        0xc402 => Some(format!("Sea: {}", value)),
        0xc403 => Some(format!("Rain: {}", value)),
        0xc404 => Some(format!("Range index: {}", value)),
        0xc405 => Some(format!(
            "Power: {}",
            if value == 1 { "ON" } else { "OFF" }
        )),
        _ => None,
    };

    (
        desc,
        serde_json::json!({
            "opcode": format!("0x{:04x}", opcode),
            "value": value
        }),
    )
}

/// Decode an RD format command.
fn decode_rd_command(lead: u8, value: u8) -> (Option<String>, serde_json::Value) {
    let desc = match lead {
        0x01 => Some(format!("Gain: {}", value)),
        0x02 => Some(format!("Sea: {}", value)),
        0x03 => Some(format!("Rain: {}", value)),
        _ => None,
    };

    (
        desc,
        serde_json::json!({
            "lead": format!("0x{:02x}", lead),
            "value": value
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
    fn test_decode_quantum_command() {
        let decoder = RaymarineDecoder;
        let msg = decoder.decode(&[0x01, 0xc4, 0x28, 0x32], IoDirection::Send);

        match msg {
            DecodedMessage::Raymarine {
                variant,
                message_type,
                ..
            } => {
                assert_eq!(variant, Some("quantum".to_string()));
                assert_eq!(message_type, "command");
            }
            _ => panic!("Expected Raymarine message"),
        }
    }

    #[test]
    fn test_decode_rd_command() {
        let decoder = RaymarineDecoder;
        let msg = decoder.decode(&[0x00, 0xc1, 0x01, 0x32, 0x00], IoDirection::Send);

        match msg {
            DecodedMessage::Raymarine {
                variant,
                message_type,
                ..
            } => {
                assert_eq!(variant, Some("rd".to_string()));
                assert_eq!(message_type, "command");
            }
            _ => panic!("Expected Raymarine message"),
        }
    }

    #[test]
    fn test_decode_empty() {
        let decoder = RaymarineDecoder;
        let msg = decoder.decode(&[], IoDirection::Recv);

        assert!(matches!(msg, DecodedMessage::Unknown { .. }));
    }
}
