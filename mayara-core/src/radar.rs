//! Radar data structures
//!
//! These structures represent radar metadata and configuration,
//! independent of any I/O or networking code.

use crate::Brand;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::net::{Ipv4Addr, SocketAddrV4};

// =============================================================================
// Serde helpers for SocketAddrV4/Ipv4Addr <-> String
// =============================================================================

mod socket_addr_serde {
    use super::*;

    pub fn serialize<S: Serializer>(addr: &SocketAddrV4, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&addr.to_string())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<SocketAddrV4, D::Error> {
        let s = String::deserialize(d)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

mod option_socket_addr_serde {
    use super::*;

    pub fn serialize<S: Serializer>(addr: &Option<SocketAddrV4>, s: S) -> Result<S::Ok, S::Error> {
        match addr {
            Some(a) => s.serialize_some(&a.to_string()),
            None => s.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<SocketAddrV4>, D::Error> {
        let opt: Option<String> = Option::deserialize(d)?;
        match opt {
            Some(s) => s.parse().map(Some).map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

mod option_ipv4_serde {
    use super::*;

    pub fn serialize<S: Serializer>(addr: &Option<Ipv4Addr>, s: S) -> Result<S::Ok, S::Error> {
        match addr {
            Some(a) => s.serialize_some(&a.to_string()),
            None => s.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Ipv4Addr>, D::Error> {
        let opt: Option<String> = Option::deserialize(d)?;
        match opt {
            Some(s) => s.parse().map(Some).map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

/// Basic radar information discovered from beacon response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadarDiscovery {
    /// Radar brand
    pub brand: Brand,
    /// Radar model (if known)
    pub model: Option<String>,
    /// Radar name/serial from beacon
    pub name: String,
    /// Primary radar address (IP + port)
    #[serde(with = "socket_addr_serde")]
    pub address: SocketAddrV4,
    /// Number of spokes per revolution
    pub spokes_per_revolution: u16,
    /// Maximum spoke length in pixels
    pub max_spoke_len: u16,
    /// Pixel depth (e.g., 16, 64, 128)
    pub pixel_values: u8,
    /// Serial number (from model report)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// NIC address that received this beacon (for multi-interface systems)
    #[serde(skip_serializing_if = "Option::is_none", with = "option_ipv4_serde")]
    pub nic_address: Option<Ipv4Addr>,
    /// Suffix for dual-range radars ("A" or "B"), None for single-range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    /// Data streaming address (for brands like Navico that use separate multicast addresses)
    #[serde(skip_serializing_if = "Option::is_none", with = "option_socket_addr_serde")]
    pub data_address: Option<SocketAddrV4>,
    /// Report/status address (for brands like Navico that use separate multicast addresses)
    #[serde(skip_serializing_if = "Option::is_none", with = "option_socket_addr_serde")]
    pub report_address: Option<SocketAddrV4>,
    /// Send/command address
    #[serde(skip_serializing_if = "Option::is_none", with = "option_socket_addr_serde")]
    pub send_address: Option<SocketAddrV4>,
}

/// Legend entry for mapping pixel values to colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegendEntry {
    /// Pixel type (Normal, TargetBorder, DopplerApproaching, etc.)
    #[serde(rename = "type")]
    pub pixel_type: String,
    /// RGBA color as hex string (e.g., "#00FF00FF")
    pub color: String,
}

/// Radar control value
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControlValue {
    /// Current value
    pub value: serde_json::Value,
    /// Whether this control is enabled
    #[serde(default)]
    pub enabled: bool,
    /// Whether this control is in auto mode
    #[serde(default)]
    pub auto: bool,
}

/// Radar control definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControlDefinition {
    /// Control name
    pub name: String,
    /// Control description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Minimum value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    /// Maximum value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    /// Step value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    /// Unit (e.g., "meters", "degrees")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Whether this control supports auto mode
    #[serde(default)]
    pub has_auto: bool,
}

/// Full radar state including controls and legend
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadarState {
    /// Radar ID
    pub id: String,
    /// Radar name
    pub name: String,
    /// Brand
    pub brand: Brand,
    /// Model (if known)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Current status
    pub status: RadarStatus,
    /// Number of spokes per revolution
    pub spokes_per_revolution: u16,
    /// Maximum spoke length
    pub max_spoke_len: u16,
    /// Legend for pixel color mapping
    pub legend: Vec<LegendEntry>,
    /// Available controls with current values
    pub controls: std::collections::HashMap<String, ControlValue>,
    /// Optional external stream URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_url: Option<String>,
}

/// Radar operational status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RadarStatus {
    /// Radar is off
    Off,
    /// Radar is warming up
    Warming,
    /// Radar is in standby mode
    Standby,
    /// Radar is transmitting
    Transmit,
    /// Radar status unknown
    Unknown,
}

impl Default for RadarStatus {
    fn default() -> Self {
        RadarStatus::Unknown
    }
}

impl std::fmt::Display for RadarStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RadarStatus::Off => write!(f, "off"),
            RadarStatus::Warming => write!(f, "warming"),
            RadarStatus::Standby => write!(f, "standby"),
            RadarStatus::Transmit => write!(f, "transmit"),
            RadarStatus::Unknown => write!(f, "unknown"),
        }
    }
}

// ParsedAddress has been removed - use std::net::SocketAddrV4 instead
