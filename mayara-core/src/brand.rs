//! Radar brand definitions

use serde::{Deserialize, Serialize, Serializer};

/// Supported radar brands
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum Brand {
    Furuno,
    Garmin,
    Navico,
    Raymarine,
}

impl Brand {
    /// Get the brand name as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Brand::Furuno => "Furuno",
            Brand::Garmin => "Garmin",
            Brand::Navico => "Navico",
            Brand::Raymarine => "Raymarine",
        }
    }
}

impl std::fmt::Display for Brand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for Brand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl TryFrom<&str> for Brand {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_ascii_lowercase().as_str() {
            "furuno" => Ok(Brand::Furuno),
            "garmin" => Ok(Brand::Garmin),
            "navico" => Ok(Brand::Navico),
            "raymarine" => Ok(Brand::Raymarine),
            _ => Err(format!("Unknown brand: {}", s)),
        }
    }
}
