# Mayara Changelog

All notable changes to the Mayara project.

## [0.1.1] - 2024-12-06

### Fixed

- **WebSocket Stream URL**: Fixed viewer.js to properly construct SignalK stream URL when `streamUrl` is undefined or literal string "undefined"
- **WebSocket Close Logging**: Added detailed close event logging (code, reason, wasClean) for debugging

### Changed

- **Radar Legend Colors**: Changed default PPI color scheme from green gradient to traditional radar colors:
  - Black (background) → Yellow (weak) → Orange (medium) → Dark Red (strong) → Bright Red (strongest returns)

---

## [Unreleased] - WASM Support Refactoring

### Overview

This release refactors mayara into a multi-crate architecture to support both standalone native execution and WASM-based SignalK plugin deployment.

### Added

#### New Crate: mayara-core

Platform-independent radar protocol library containing:

- **Protocol Parsing**: Pure `&[u8]` → `Result<T>` parsing for all radar brands
  - Furuno beacon/report parsing
  - Navico beacon/report parsing
  - Raymarine beacon/report parsing
  - Garmin beacon/report parsing
- **Data Structures**: RadarInfo, Legend, Controls, Spoke (no I/O dependencies)
- **Constants**: Port numbers, packet headers, broadcast addresses
- **Network Requirements**: IP range validation for brand-specific network requirements
  - `furuno::is_valid_furuno_ip()` - Check if host IP is in required 172.31.x.x range
  - `furuno::network_requirement_message()` - User-friendly configuration help
- **Protobuf**: RadarMessage encoding for spoke data

#### New Crate: mayara-signalk-wasm

SignalK WASM plugin that:

- Uses `mayara-core` for protocol parsing
- Integrates with SignalK's `rawSockets` capability for UDP
- Registers as SignalK Radar Provider
- Supports optional external `streamUrl` for data plane separation
- Combined plugin + webapp package (`@mayara/signalk-radar`)
- **Cross-platform build script** (`build.js`): Works on Windows, Linux, and macOS
  - `npm run build` - Build WASM plugin
  - `npm run build:test` - Run tests + build
  - `npm run pack` - Build + create `.tgz` package

#### WebApp Improvements

- **Network Configuration Help**: When no radars are detected, displays expandable help section with:
  - Furuno DRS IP range requirements (172.31.x.x/16 - hardwired in radar hardware)
  - Navico/Raymarine/Garmin multicast requirements
  - Example configuration commands

### Changed

#### mayara-lib Refactoring

- Now depends on `mayara-core` for protocol logic
- Retains async/tokio runtime for native execution
- Platform-specific networking code unchanged
- Re-exports `mayara-core` types for API compatibility

### Architecture

```
mayara/
├── mayara-core/          # Protocol parsing, no I/O (WASM-compatible)
├── mayara-lib/           # Native async runtime (tokio, sockets)
├── mayara-server/        # Standalone HTTP/WebSocket server
└── mayara-signalk-wasm/  # SignalK WASM plugin + webapp
    ├── src/              # Rust WASM code
    ├── public/           # WebApp (HTML, JS, CSS)
    └── package.json      # npm package config
```

### Migration Notes

- Existing `mayara-lib` API remains compatible
- `mayara-server` works unchanged
- New WASM plugin provides SignalK integration

---

## [0.3.0] - Previous Release

See git history for changes prior to WASM refactoring.
