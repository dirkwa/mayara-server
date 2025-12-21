# Protocol Debugger User Guide

The Protocol Debugger is a development tool for analyzing and reverse-engineering marine radar protocols. It captures network traffic, decodes protocol messages, and helps identify unknown protocol elements.

> **Important:** This feature is only available when built with `--features dev`.

---

## Important Limitations

### What We Can See

| Traffic | Visible | Why |
|---------|---------|-----|
| mayara-server → radar | ✅ Yes | Goes through our sockets |
| radar → mayara-server | ✅ Yes | Received by our sockets |
| chart plotter → radar | ❌ No | Direct connection, bypasses us |
| radar → chart plotter | ⚠️ Partial | Multicast traffic visible, TCP not visible |

**Why this matters:** When you press a button on your Garmin/Simrad/Furuno MFD, the command goes directly from the chart plotter to the radar. We don't see the command, but for multicast protocols (Navico, Garmin, Raymarine) we can see the radar's status broadcasts change as a result.

### Capturing Chart Plotter Commands

To capture chart plotter commands directly, use `tcpdump`:

```bash
# Capture all radar traffic on interface eth0
sudo tcpdump -i eth0 -w radar-capture.pcap host 172.31.1.4

# Capture specific ports (Furuno control)
sudo tcpdump -i eth0 -w furuno.pcap 'host 172.31.1.4 and port 10000'

# Capture multicast groups (Navico)
sudo tcpdump -i eth0 -w navico.pcap 'multicast and net 236.6.7.0/24'
```

Then analyze in Wireshark with the radar protocol documentation.

---

## Getting Started

### Prerequisites

- mayara-server built with `--features dev`
- A radar connected (or `.mrr` recording for playback)

### Enable the Debugger

```bash
# Build with dev features
cd /home/dirk/dev/mayara-server
cargo build -p mayara-server --features dev
./target/debug/mayara-server
```

### Open the Debug Panel

1. Open `http://localhost:6502/` in your browser
2. Click the **Debug** icon (🔬) in the toolbar (only visible in dev mode)
3. The debug panel appears as a collapsible sidebar

---

## Debug Panel Overview

### Radar Status Cards

Shows all connected radars with:
- Brand and model
- Connection state (green=connected, yellow=connecting, red=error)
- IP address

### Event Timeline

Real-time scrollable list of network events:
- **Blue**: Data sent (TCP/UDP)
- **Green**: Data received
- **Orange**: Socket operations (connect, bind, close)
- **Purple**: State changes
- **Red/Orange**: Unknown/unparseable messages

Click an event to see details in the Packet View.

### Packet View

When an event is selected:
- **Hex dump**: Raw bytes with offset
- **ASCII view**: Printable characters (dots for non-printable)
- **Decoded fields**: Parsed protocol structure

For unknown bytes, regions are highlighted as `[UNKNOWN - N bytes]`.

### State Change View

Shows before/after comparison when radar state changes:
- Which control changed
- Previous and new values
- Triggering event (if correlatable)

---

## REST API

The debug feature adds these endpoints:

### WebSocket: Real-time Events
```
GET /v2/api/debug
```

Connect via WebSocket to receive real-time events.

**Client→Server messages:**
```json
{"type": "subscribe", "radarId": "radar-1"}  // Filter by radar
{"type": "getHistory", "limit": 100}         // Get historical events
{"type": "pause"}                             // Pause streaming
{"type": "resume"}                            // Resume streaming
```

**Server→Client messages:**
```json
{"type": "connected", "eventCount": 1234}
{"type": "event", ...}
{"type": "history", "events": [...]}
```

### Query Events
```
GET /v2/api/debug/events?radar_id=radar-1&limit=100&after=500
```

Returns historical events with optional filtering.

### Recording Control
```
POST /v2/api/debug/recording/start
Body: {"radars": [{"radarId": "radar-1", "brand": "furuno"}]}

POST /v2/api/debug/recording/stop

GET /v2/api/debug/recordings
```

---

## Workflow: Discovering Unknown Protocol Elements

### Step 1: Start Observing

1. Open the debug panel
2. Start mayara-server with a radar connected
3. Observe the initial handshake and status messages

### Step 2: Trigger Actions on Chart Plotter

1. Press a button on your chart plotter (e.g., change gain)
2. Watch the Event Timeline for new messages
3. Note the timestamp

### Step 3: Correlate Changes

For multicast protocols:
- You'll see a status broadcast showing the new state
- The debugger correlates this with recent commands

For TCP protocols (Furuno):
- If the chart plotter triggered it, you won't see the command
- Use `tcpdump` to capture traffic (see below)

### Step 4: Record and Export

1. Click **Start Recording** in the debug panel
2. Perform actions on the chart plotter
3. Click **Stop Recording**
4. Add annotations (e.g., "Pressed Bird Mode at 14:30:45")
5. Export as `.mdbg` file for sharing

---

## Using tcpdump for Full Traffic Capture

Since the debugger can't see chart plotter → radar traffic directly, use `tcpdump`:

### Furuno (TCP on 172.31.x.x)

```bash
# Find radar IP
ip neigh | grep 172.31

# Capture all traffic to/from radar
sudo tcpdump -i eth0 -w furuno-session.pcap host 172.31.1.4

# In another terminal, use your chart plotter
# When done, Ctrl+C to stop capture

# Analyze in Wireshark
wireshark furuno-session.pcap
```

### Navico (UDP Multicast)

```bash
# Capture all Navico multicast traffic
sudo tcpdump -i eth0 -w navico.pcap 'multicast and (net 236.6.7.0/24 or net 239.238.55.0/24)'
```

### Raymarine (UDP Multicast)

```bash
# Capture Raymarine traffic
sudo tcpdump -i eth0 -w raymarine.pcap 'multicast and net 224.0.0.0/4 and port 5800'
```

### Garmin (UDP Multicast)

```bash
# Capture Garmin traffic
sudo tcpdump -i eth0 -w garmin.pcap 'multicast and net 239.254.2.0/24'
```

---

## Session Recording Format

`.mdbg` files are JSON and contain:
- All debug events with timestamps
- Radar capabilities and state at recording time
- User annotations
- mayara-server version

Files can be loaded by any developer to replay and analyze.

### Recording Structure

```json
{
  "metadata": {
    "formatVersion": 1,
    "startTime": "2024-01-15T14:30:22Z",
    "endTime": "2024-01-15T14:35:45Z",
    "serverVersion": "0.6.0",
    "radars": [
      {"radarId": "radar-1", "brand": "furuno", "model": "DRS4D-NXT"}
    ],
    "eventCount": 1234,
    "annotations": [
      {"timestamp": 123456, "note": "Pressed bird mode button"}
    ]
  },
  "events": [...]
}
```

---

## Tips for Effective Reverse Engineering

1. **Start with known operations**: First observe commands you already understand
2. **One change at a time**: Change one setting, observe the result
3. **Document timestamps**: Note exactly when you press each button
4. **Combine tools**: Use Protocol Debugger + tcpdump together
5. **Share recordings**: Upload `.mdbg` files to issues for collaboration

---

## Troubleshooting

### "No debug events appearing"

- Verify `--features dev` was used at compile time
- Check that radars are connected and transmitting
- Look at the Radar Status card for connection state

### "Can't see chart plotter commands"

This is expected. Use `tcpdump` for full traffic capture.

### "Decoded fields are empty"

The decoder may not recognize the message format. The raw hex is always available. Consider contributing to the protocol documentation.

### "Recording file is empty"

- Ensure you called "Start Recording" before performing actions
- Check that events were flowing during the recording period
- Verify disk space is available

---

## Protocol-Specific Notes

### Furuno

- Uses ASCII commands over TCP (e.g., `$S69,50\r\n`)
- Commands start with `$S` (set), `$R` (read), `$N` (notification)
- The decoder recognizes common commands like gain, sea, rain

### Navico (Simrad, B&G, Lowrance)

- Uses binary UDP multicast
- Spoke data, status reports, and commands have different structures
- Large packets (>100 bytes) are typically spoke data

### Raymarine

- Uses binary UDP multicast
- Beacon packets are 56 bytes
- Different radar variants use different packet formats

### Garmin

- Uses binary UDP multicast on 239.254.2.x
- Status and control messages are relatively small
- Spoke data arrives in larger packets

---

## See Also

- [Getting Started](../develop/getting_started.md) - Development environment setup
- [Building](../develop/building.md) - Build commands and feature flags
- [Architecture](../design/architecture.md) - System design overview
