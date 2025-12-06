# mayara-core

Platform-independent radar protocol library for the Mayara project.

## Purpose

This crate contains pure parsing logic for marine radar protocols. It has **no I/O dependencies** and can be compiled for any target including WebAssembly.

## Supported Radars

| Brand | Models | Status |
|-------|--------|--------|
| **Furuno** | DRS4D-NXT, DRS6A-NXT, DRS12A-NXT, FAR series | ✅ Implemented |
| **Navico** | BR24, 3G, 4G, HALO series | 🚧 TODO |
| **Raymarine** | Quantum, RD series | 🚧 TODO |
| **Garmin** | xHD series | 🚧 TODO |

## Usage

```rust
use mayara_core::protocol::furuno;
use mayara_core::Brand;

// Parse a beacon response
let packet: &[u8] = &[/* radar response bytes */];
match furuno::parse_beacon_response(packet, "172.31.6.1") {
    Ok(discovery) => {
        println!("Found {} radar: {}", discovery.brand, discovery.name);
        println!("  Address: {}", discovery.address);
        println!("  Spokes: {}", discovery.spokes_per_revolution);
    }
    Err(e) => println!("Parse error: {}", e),
}

// Create beacon request packet
let request = furuno::create_beacon_request();
// ... send via UDP to 172.31.255.255:10010
```

## Design Principles

1. **No I/O**: All functions are pure - they take `&[u8]` and return parsed data
2. **No async**: No tokio, no futures - just synchronous parsing
3. **No platform code**: No `#[cfg(target_os)]`, no system calls
4. **Minimal dependencies**: Only serde, bincode, thiserror

## Crate Structure

```
mayara-core/
├── src/
│   ├── lib.rs           # Crate root, re-exports
│   ├── brand.rs         # Brand enum (Furuno, Navico, etc.)
│   ├── error.rs         # ParseError types
│   ├── radar.rs         # RadarDiscovery, RadarState, etc.
│   └── protocol/
│       ├── mod.rs       # Protocol module, helpers
│       ├── furuno.rs    # Furuno protocol constants & parsing
│       ├── navico.rs    # Navico protocol (TODO)
│       ├── raymarine.rs # Raymarine protocol (TODO)
│       └── garmin.rs    # Garmin protocol (TODO)
```

## Feature Flags

Individual radar brands can be enabled/disabled:

```toml
[dependencies]
mayara-core = { version = "0.1", default-features = false, features = ["furuno"] }
```

Available features:
- `furuno` (default)
- `navico` (default)
- `raymarine` (default)
- `garmin` (default)

## Relationship to Other Crates

```
mayara-core     # This crate - protocol parsing
    ↑
    ├── mayara-lib    # Native runtime (tokio, real sockets)
    │       ↑
    │       └── mayara-server  # Standalone HTTP/WebSocket server
    │
    └── mayara-wasm   # SignalK WASM plugin (uses SignalK FFI sockets)
```

## License

Apache-2.0
