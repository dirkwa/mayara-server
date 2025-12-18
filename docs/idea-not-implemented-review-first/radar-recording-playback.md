# Radar Recording & Playback - Proposal for Review

**Status:** Proposal - Not Yet Implemented
**Author:** Claude Code (with Dirk)
**Date:** 2025-12-18

## Summary

Add recording and playback functionality to mayara-server with two access paths:

1. **mayara-server (Rust)** - Records and plays back standalone
2. **SignalK plugin (JS)** - Can also play `.mrr` files via SignalK Radar API

## Use Cases

- **SignalK developers** - Test `render()` functions with consistent recorded data
- **OpenCPN plugin** (future) - Replay recordings for development/testing
- **Demos/exhibitions** - Run standalone without SignalK or real radar
- **Bug reports** - Users can share recordings to reproduce issues
- **Any SignalK radar client** - Benefits from playback via standard Radar API

## User Requirements

- Simple binary file format (`.mrr` - MaYaRa Radar Recording)
- Configurable storage path (default: `~/.local/share/mayara/recordings/`)
- Works standalone (mayara-server only) AND with SignalK
- SignalK plugin can play `.mrr` files directly (no mayara-server needed for playback)
- Web GUI shows playback with disabled controls (read-only state display)
- Full metadata recording (capabilities, controls, range changes)
- Single radar recording at a time

## Architecture

```
                           RECORDING PATH
  ┌──────────────────────────────────────────────────────────────────┐
  │                      mayara-server (Rust)                         │
  │  ┌─────────────┐    ┌─────────────┐    ┌──────────────────────┐  │
  │  │Radar Drivers│───►│  Recorder   │───►│  ~/.../recordings/   │  │
  │  │(Furuno,etc) │    │             │    │  *.mrr               │  │
  │  └─────────────┘    └─────────────┘    └──────────────────────┘  │
  └──────────────────────────────────────────────────────────────────┘

                         PLAYBACK PATHS (2 options)

  Option A: Standalone (mayara-server only)
  ┌──────────────────────────────────────────────────────────────────┐
  │  mayara-server ─► Player ─► Virtual Radar ─► mayara-gui          │
  │                                                                   │
  │  Good for: demos, exhibitions, testing without SignalK           │
  └──────────────────────────────────────────────────────────────────┘

  Option B: SignalK (for radar API consumers)
  ┌──────────────────────────────────────────────────────────────────┐
  │  .mrr file ─► SignalK Plugin ─► radarApi.register() ─► SignalK   │
  │                    │                                        │     │
  │                    │            binaryStreamManager         │     │
  │                    └───────────────────────────────────────►│     │
  │                                                             ▼     │
  │                                           ┌─────────────────────┐ │
  │                                           │  Any Radar Consumer:│ │
  │                                           │  - mayara-gui       │ │
  │                                           │  - OpenCPN (future) │ │
  │                                           │  - SignalK devs     │ │
  │                                           └─────────────────────┘ │
  │                                                                   │
  │  Good for: SignalK developers testing render(), chart plotters   │
  └──────────────────────────────────────────────────────────────────┘

                         mayara-gui (both paths)

  Normal Mode:              Playback Mode:
  - Live radar              - Virtual radar (playback-*)
  - Controls ENABLED        - Controls DISABLED (read-only)
  - User can change         - Shows recorded state
                            - "PLAYBACK" badge on header
                            - Timeline slider for seeking
```

## Key Design Decisions

### Why record in mayara-server (Rust)?

- Closest to the data source - most efficient
- No network overhead or latency
- Access to raw RadarMessage protobuf before any transformation
- Can record even without SignalK running

### Why playback in both mayara-server AND SignalK plugin?

- **mayara-server playback**: Standalone demos, testing without SignalK
- **SignalK plugin playback**: Registers as RadarProvider, making recordings available to ANY SignalK radar consumer (including future OpenCPN plugin)

### No changes to SignalK Radar API needed

Playback just registers as a regular RadarProvider - existing consumers work automatically.

## File Format (.mrr)

Simple binary format using existing protobuf for spokes:

```
┌──────────────────────────┐
│ Header (256 bytes)       │  magic "MRR1", version, radar metadata
├──────────────────────────┤
│ Capabilities (JSON)      │  length-prefixed JSON (v5 capabilities)
├──────────────────────────┤
│ Initial State (JSON)     │  length-prefixed JSON (controls state)
├──────────────────────────┤
│ Frame 0                  │  timestamp + protobuf RadarMessage + state delta
│ Frame 1                  │
│ ...                      │
├──────────────────────────┤
│ Index (for seeking)      │  array of (timestamp, file_offset)
├──────────────────────────┤
│ Footer (32 bytes)        │  index offset, frame count, duration
└──────────────────────────┘
```

**Estimated file sizes:** ~15-30 MB/minute, ~1-2 GB/hour

## New REST API Endpoints

All at `/v2/api/recordings/`:

### Recording Control
```
GET  /v2/api/recordings/radars          # List available radars to record
POST /v2/api/recordings/record/start    # {radarId, filename?}
POST /v2/api/recordings/record/stop
GET  /v2/api/recordings/record/status
```

### Playback Control
```
POST /v2/api/recordings/playback/load   # {filename}
POST /v2/api/recordings/playback/play
POST /v2/api/recordings/playback/pause
POST /v2/api/recordings/playback/stop
POST /v2/api/recordings/playback/seek   # {timestamp_ms}
PUT  /v2/api/recordings/playback/settings  # {loop?, speed?}
GET  /v2/api/recordings/playback/status
```

### File Management
```
GET    /v2/api/recordings/files              # ?dir=subdir
GET    /v2/api/recordings/files/:filename
DELETE /v2/api/recordings/files/:filename
PUT    /v2/api/recordings/files/:filename    # {newName?, directory?}
POST   /v2/api/recordings/files/upload
GET    /v2/api/recordings/files/:filename/download
POST   /v2/api/recordings/directories        # {name}
DELETE /v2/api/recordings/directories/:name
```

## New Files in mayara-server

```
mayara-server/
├── src/
│   ├── recording/
│   │   ├── mod.rs           # Module exports
│   │   ├── recorder.rs      # Recorder: subscribes to broadcast, writes .mrr
│   │   ├── player.rs        # Player: reads .mrr, emits to broadcast as virtual radar
│   │   ├── file_format.rs   # .mrr binary format read/write
│   │   └── manager.rs       # File listing, metadata, CRUD
│   └── web.rs               # Add /v2/api/recordings/* endpoints
└── ...
```

## GUI Changes (mayara-gui)

1. **Playback detection** - Detect if viewing a playback radar (ID starts with "playback-")
2. **Disable controls** - All controls disabled for playback radars, show "PLAYBACK" badge
3. **New recordings page/tab** - Record, Playback, and File management UI

## Implementation Phases

### Phase 1: File Format & Storage (Rust)
- Create `recording/` module structure
- Implement `.mrr` binary format read/write
- Implement file listing and metadata extraction
- Add file management endpoints

### Phase 2: Recording (Rust)
- Implement recorder (subscribe to radar broadcast, write frames)
- Add virtual radar ID tracking
- Add recording control endpoints

### Phase 3: Playback (Rust)
- Implement player (read frames, emit at correct timing)
- Register playback as "virtual radar" in radar list
- Add playback control endpoints

### Phase 4: GUI Updates (JavaScript)
- Add playback detection in `api.js`
- Disable controls for playback radars in `control.js`
- Create recordings UI with recorder/playback/file browser

### Phase 5: SignalK Integration (Optional)
- mayara-server-signalk-plugin can proxy recording/playback APIs
- Or: SignalK plugin plays `.mrr` files directly (registers as RadarProvider)

## Questions for Discussion

1. **File format**: Is the proposed `.mrr` format appropriate, or should we use something more standard?

2. **Storage location**: Is `~/.local/share/mayara/recordings/` the right default?

3. **Playback speed**: Should we support variable speed playback (0.5x, 2x, etc.)?

4. **Multi-radar**: Should we support recording multiple radars simultaneously in the future?

5. **Compression**: Should frames be compressed (e.g., zstd)?

6. **SignalK-only playback**: Is it valuable for the SignalK plugin to play `.mrr` files independently (without mayara-server)?

## Related

- Existing `--replay` mode uses tcpreplay for network-level replay
- This proposal is higher-level: application-level recording with full state
