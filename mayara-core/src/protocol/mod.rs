//! Radar protocol implementations
//!
//! Each module contains constants and pure parsing functions for a specific brand.

#[cfg(feature = "furuno")]
pub mod furuno;

#[cfg(feature = "navico")]
pub mod navico;

#[cfg(feature = "raymarine")]
pub mod raymarine;

#[cfg(feature = "garmin")]
pub mod garmin;

/// Helper function to extract a null-terminated C string from bytes
pub fn c_string(bytes: &[u8]) -> Option<String> {
    let null_pos = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    std::str::from_utf8(&bytes[..null_pos])
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_string() {
        assert_eq!(c_string(b"hello\0world"), Some("hello".to_string()));
        assert_eq!(c_string(b"hello"), Some("hello".to_string()));
        assert_eq!(c_string(b"\0"), None);
        assert_eq!(c_string(b"  test  \0"), Some("test".to_string()));
    }
}
