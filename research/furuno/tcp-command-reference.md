# Furuno DRS TCP Command Reference

Derived from:
- Decompilation of `Fec.FarApi.dll` (`RadarCommandID` enum, `RmcSet*`/`RmcGet*` functions)
- Decompilation of `MaxSea.Radar.dll` (capability tables, per-model feature detection)
- TCP session captures from DRS4D-NXT and FAR-2127 (`research/furuno/*.md`)

## Protocol Format

```
$S<hex_id>,<params>     Set command (client → radar)
$R<hex_id>,<params>     Request current value (client → radar)
$N<hex_id>,<params>     Response/notification (radar → client)
```

The hex ID is `0x60 + RadarCommandID` enum value, transmitted as uppercase hex.

## Complete Command Table

### Core Radar Controls

| Hex | Name | Set Format | Response Format | Description |
|-----|------|-----------|-----------------|-------------|
| 60/00 | FreeCommand | `$S00,<name>,<text>` | `$N00,<name>,<value>` | Named key-value pairs (Fan1Status, TILEEAV, etc.) |
| 61 | DispMode | `$S61,<status>,<dir>,<screen>` | `$N61,<status>,<dir>,<screen>` | Display mode (head-up/north-up/course-up) |
| 62 | Range | `$S62,<wire_idx>,<unit>,<drid>` | `$N62,<wire_idx>,<unknown>,<unit>` | Range selection |
| 63 | Gain | `$S63,<auto>,<val>,<screen>,<auto_val>,<drid>` | `$N63,<auto>,<val>,<screen>,<auto_val>,<drid>` | Gain control |
| 64 | Sea | `$S64,<auto>,<val>,<auto_val>,<screen>,0,<drid>` | `$N64,<auto>,<val>,<auto_val>,<screen>,0,<drid>` | Sea clutter |
| 65 | Rain | `$S65,<auto>,<val>,0,<screen>,<drid>,0` | `$N65,<auto>,<val>,0,<screen>,<drid>,0` | Rain clutter |
| 66 | CustomPictureAll | `$R66` or `$R66,99` | `$N66,<26 values>` | Query all 26 signal processing features |
| 67 | CustomPicture | `$S67,0,<feat>,<val>,<screen>` | `$N67,0,<feat>,<val>,<screen>` | Individual signal processing feature |
| 68 | PulseWidth | `$S68,<pulse>,<range>,<unit>,<imgNo>,<screen>` | `$N68,<pulse>,<range>,<unit>,<imgNo>,<screen>` | Pulse width selection |
| 69 | TxSTBY | `$S69,<status>,<wman>,<wsend>,<wstop>,0,<drid>` | `$N69,<status>,<wman>,<wsend>,<wstop>,0,<drid>` | TX/Standby + watchman |

`<drid>` = dual_range_id (0=Range A, 1=Range B). `<screen>` also used for dual range in some commands.

### ARPA/Target Tracking

| Hex | Name | Format | Description |
|-----|------|--------|-------------|
| 6A | SelectTarget | `<x>,<y>,<number>` | ARPA target selection |
| 6B | ACQTarget | `<x>,<y>,<mode>` | Acquire ARPA target |
| 6C | CancelTarget | `<x>,<y>` | Cancel ARPA target |
| 6D | ARPADispMode | `<status>,<mode>,<vector>,<autop>,<manualp>,<trackBase>` | ARPA display config |
| 70 | GuardStatus | `<count>,<status0>,<status1>` | Guard zone status |
| 73 | ARPAVector | `<mode>,<time>` | ARPA vector mode |
| 74 | ARPACpaTcpa | `<sw>,<cpa>,<tcpa>,<d_cpa>,<d_tcpa>` | CPA/TCPA thresholds |
| 79 | ArpaAllClear | (no args) | Clear all ARPA targets |
| 7B | CancelTargetID | `<targetNo>` | Cancel by target ID |
| 97 | StartArpaTest | `<status>` | ARPA test |
| 98 | GuardSelect | `<zoneNo>,<zone_stab>,<polygon>` | Guard zone mode |
| 99 | GuardArea1 | `<zoneNo>,<startDist>,<startDir>,<endDist>,<endDir>` | Guard zone fan |
| 9A | GuardArea2 | `<zoneNo>,<total>,<dist[]>,<dir[]>,...` | Guard zone polygon |
| 9C | TTGyro | `<select>` | TT without gyro |
| BC | ArpaMaxRange | `<maxRange>` | ARPA max range |
| BD | EchoLevel | `<level>` | ARPA echo level |
| BF | LandSize | `<size>` | ARPA land size |
| C0 | ArpaAntenna | `<kind>` | ARPA antenna select |
| C1 | ArpaAcqCorre | `<scan>` | ARPA correlation |
| C2 | ArpaAcqWeed | `<weed>` | ARPA weed filter |
| C3 | ArpaGateSize | `<size>` | ARPA gate size |
| C4 | ArpaFilterResp | `<res>` | ARPA filter response |
| C5 | ArpaLostCount | `<count>` | ARPA lost count |
| C7 | ArpaTimeVector | `<mode>,<count>` | ARPA time vector |
| D0 | ArpaLostFilterRange | `<status>,<rangeNM>,<range>` | Lost filter range |
| D1 | ArpaLostFilterSpeed | `<status>,<speed>` | Lost filter speed |
| D7 | ArpaLostFilterMode | `<mode>` | Lost filter mode |
| F0 | AccuShip | `<status>` | Auto-acquire (by Doppler on NXT) |
| FB | ARPADetect | `<range>,<level>,<size>` | ARPA detection params |
| FC | ARPACatch | `<landsize>,<correl>,<cancel>,<gate>` | ARPA catch params |
| FD | ARPAPursuit | `<gate>,<filter>,<lost>,<speed>,<mode>,<unit>,<data>` | ARPA pursuit (7 params) |

### Antenna & Hardware

| Hex | Name | Format | Description |
|-----|------|--------|-------------|
| 6E | AntennaType | `<type>,<pos>,<output>,<length>,<updown>,<atype>,<model2>` | Antenna info (7 params) |
| 77 | BlindSector | `<s2_en>,<s1_start>,<s1_width>,<s2_start>,<s2_width>` | No-transmit sectors |
| 83 | MBSAdjust | `<mbs>,<pulse>` | Main bang suppression (0-255, pulse) |
| 84 | AntennaHeight | `<monitorNo>,<height>,<height2>` | Antenna height |
| 85 | NearSTC | `<curve>` | Near STC curve |
| 86 | MiddleSTC | `<curve>` | Mid STC curve |
| 87 | FarSTC | `<curve>` | Far STC curve |
| 88 | RingSuppression | `<data>` | Ring suppression |
| 89 | AntennaRevolution | `<speed>,<highRotationMode>` | Scan speed |
| 8A | AntennaSW | `<sw>` | Antenna power switch |
| 8D | AntennaNo | `<ant>` | Antenna number |
| D2 | StcRange | `<range>` | STC range |

### Timing & Tuning

| Hex | Name | Format | Description |
|-----|------|--------|-------------|
| 75 | Tune | `<status>,<tune>,<screen>` | Tuning (auto/manual, value) |
| 76 | TuneIndicator | `<tune_volt>,<errFlag>,<screen>` | Tune readback |
| 7A | TuneAdjust | `<status>` | Tune adjustment |
| 80 | ATT | `<monNo>,<status>,<manual>` | Auto tune/timing |
| 81 | HeadingAdjust | `<monNo>,<dir>` | Heading alignment |
| 82 | TimingAdjust | `<monNo>,<mode>,<timing>,<autoData>,<offset>` | Timing alignment |

### NXT-Specific Features

| Hex | Name | Format | Description |
|-----|------|--------|-------------|
| EA | AtfSettings | `<adjust>,<highGain>,<cutLevel>,<startDir>,<sector>,<screen>` | ATF settings |
| ED | BirdMode | `<mode>,<screen>` | High sensitivity (0=Off, 1-3=Low/Med/High) |
| EE | RezBoost | `<mode>,<screen>` | Beam sharpening (0=Off, 1-3=Low/Med/High) |
| EF | TargetAnalyzer | `<enabled>,<mode>,<screen>` | Doppler (0=Off; en=1 mode=0=Target, mode=1=Rain) |
| EC | SsdTxChannel | `<channel>` | SSD TX channel |
| E4 | TxEchoSelect | `<select>,<screen>` | TX echo source |

### Trail/Echo

| Hex | Name | Format | Description |
|-----|------|--------|-------------|
| A3 | TrailMode | `<mode>,<grad>,<narrow>,<level>,<range>,<copy>,<os>,<time>` | Trail settings (8 params) |
| D4 | BuildUpTime | `<buildUpTime>` | Trail build-up time |
| E0 | AtfParameter | `<pictNo>,<mode>,<gain>,<sea>,<rain>,<hGain>,<mode2>,<screen>` | ATF custom params (8 params) |
| E1 | TrailProcess | `<mode>` | Trail processing mode |
| 93 | 2ndEchoReject | `<select>` | Second echo rejection / overlay mode |

### System & Diagnostics

| Hex | Name | Format | Description |
|-----|------|--------|-------------|
| 8E | OnTime | `<seconds>` | Operating time (read-only) |
| 8F | TxTime | `<seconds>` | Transmit time (read-only) |
| 96 | Modules | `<app>,<start>,<boot>,<fpga>,...` | Firmware version strings |
| AF | ARPAAlarm | `<value>` | Heartbeat / ARPA alarm — `$NAF,256` sent frequently |
| D5 | DisplayUnitInfo | `<antNo>,<master>,<hostname>` | Display unit registration |
| E3 | AliveCheck | (no args) | Keepalive ping |
| E5 | RxEchoSelect | `<select>,<ip>` | RX echo select with IP |
| F5 | NN3Command | `<sub>,<p1>,<p2>,<p3>,<p4>` | Hardware diagnostics (frequent!) |

### Navigation & Position

| Hex | Name | Format | Description |
|-----|------|--------|-------------|
| 7E | Heading | `<mode>,<heading>` | Heading source/value |
| 9D | Speed | `<mode>,<speed>` | Speed source |
| 9E | Drift | `<mode>,<set>,<drift>` | Set/drift |
| A6 | OwnPos | `<x>,<y>` | Own ship position |
| A9 | AntennaPos | `<antNo>,<bow>,<port>` | GPS antenna position |
| AA | ConningPos | `<bow>,<port>` | Conning position |
| AD | ShipInfo | `<length>,<width>` | Ship dimensions |

## Signal Processing Features (Command 0x67)

Command 0x67 controls 27 individual signal processing features. Format:
- Request: `$R67,0,<feature_id>,,<screen>`
- Response: `$N67,0,<feature_id>,<value>,<screen>`
- Set: `$S67,0,<feature_id>,<value>,<screen>`

The `<screen>` parameter (0 or 1) selects dual range: 0=Range A, 1=Range B.

| ID | Name | DRS4D-NXT Default | Description |
|----|------|-------------------|-------------|
| 0 | InterferenceReject | 0 | Interference rejection (0=Off, 2=On) |
| 1 | EchoStretch | 0 | Echo stretch |
| 2 | EchoAverage | 0 | Echo averaging |
| 3 | NoiseReject | 0 | Noise rejection (0=Off, 1=On) |
| 4 | AutoSTC | 0 | Automatic STC |
| 5 | AutoRain | 0 | Automatic rain clutter |
| 6 | VideoContrast | 3 | Video contrast level |
| 7 | Pulse1 | 0 | Pulse width for range step 1 |
| 8 | Pulse2 | 1 | Pulse width for range step 2 |
| 9 | Pulse3 | 1 | Pulse width for range step 3 |
| 10 | Pulse4 | 1 | Pulse width for range step 4 |
| 11 | Pulse5 | 0 | Pulse width for range step 5 |
| 12 | Pulse6 | 2 | Pulse width for range step 6 |
| 13 | SeaCondition | 0 | Sea condition preset |
| 14 | AntennaHeight | 0 | Antenna height (feature context) |
| 15 | STCRange | 0 | STC range |
| 16-20 | PulseS1-S5 | 0 | S-band pulse widths (FAR series) |
| 21 | Wiper | 0 | Wiper mode |
| 22 | SCRatio | 0 | Signal/clutter ratio |
| 23 | NearSTCCurve | 0 | Near STC curve |
| 24 | LowLevelEcho | 0 | Low level echo enhancement |
| 25 | TTEchoLevel | 0 | Target tracking echo level |
| 26 | (Extension) | 5 | Extended feature (seen in capture as value 5) |

## Per-Model Capability Matrix

From `MaxSea.Radar.dll` capability table (`RCpREUlX8k1` method):

| Capability | DRS4D-NXT | DRS6A-NXT | DRS12A/25A | DRS | DRS4DL | DRS6A-XC | FAR-21x7 | FAR-15x3 | FAR-3000 |
|-----------|-----------|-----------|------------|-----|--------|----------|----------|----------|----------|
| ManualGain | Y | Y | Y | Y | Y | Y | Y | Y | Y |
| AutoGain | Y | Y | Y | Y | - | Y | - | - | - |
| ManualSea | Y | Y | Y | Y | Y | Y | Y | Y | Y |
| AutoSea | Y | Y | Y | Y | Y | Y | - | Y | Y |
| ManualRain | Y | Y | Y | Y | Y | Y | Y | Y | Y |
| AutoRain | Y | Y | Y | Y | Y | Y | - | Y | Y |
| RezBoost | Y | Y | Y | - | - | - | - | - | - |
| TargetAnalyzer | Y | Y | Y | - | - | - | - | - | - |
| TargetAnalyzerRain | Y | Y | Y | - | - | - | - | - | - |
| BirdMode | Y | Y | Y | - | - | Y | - | - | - |
| NoiseRejection | Y | Y | Y | - | - | - | - | Y | Y |
| InterferenceReject | 2-level | 2-level | 2-level | - | 1-level | - | 4-level | 4-level | 4-level |
| ScanSpeed | Y | Y | Y | - | - | - | - | - | - |
| SectorBlanking | Y | Y | Y | - | - | - | - | - | - |
| DualSectorBlanking | Y | Y | Y | - | - | - | - | - | - |
| MainBangSuppression | Y | Y | Y | - | - | - | - | - | - |
| DualRange | Y | Y | Y | - | - | - | - | - | - |
| ARPA | Y | Y | Y | Y | Y | Y | Y | Y | Y |
| AutoARPAByDoppler | Y | Y | Y | - | - | - | - | - | - |
| Watchman | Y | Y | Y | - | - | - | - | - | - |
| TxChannel (SSD) | Y | Y | Y | - | - | - | - | - | - |
| PulseLength | - | - | - | - | - | - | Y | Y | Y |
| AceMode | - | - | - | - | - | - | - | Y | Y |

## Observations from Session Captures

### DRS4D-NXT Session (`drs4dnxt-command-1.md`)

1. **Dual range initialization**: After the initial status queries, the session shows
   commands with `screen=1` (e.g., `$R67,0,0,,1`, `$R63,0,0,1,0,0`) — confirming
   dual range is queried on both screens.

2. **$NAF,256**: Very frequent heartbeat, appears after most exchanges.

3. **$NF5,<sub>,<value>,0,0,0**: NN3 diagnostic — appears between range changes.
   `sub=3` and `sub=4` alternate, values around 494-498 and 1195-1198.
   Likely magnetron/hardware telemetry.

4. **Range cycling**: The capture shows `$S62,<idx>,0,0` for all wire indices
   21,0,1,2,...15 — a complete range sweep. Each range change triggers `$N83,128,<val>`
   (main bang auto-adjust per range).

5. **$N83 auto-adjust**: Main bang suppression changes automatically with range:
   range 0-1 → mbs=0, range 2-4 → mbs=1, range 5-6 → mbs=2, range 7-8 → mbs=3,
   range 9-11 → mbs=4, range 12-15 → mbs=5.

6. **Gain response pattern**: Setting `$S63,0,50,0,80,0` echoes back as both
   `$N63,0,50,0,80,0` (screen 0) AND `$N63,0,50,1,80,0` (screen 1) — the radar
   mirrors per-range settings to both screens.

### FAR-2127 Session (`far2127-command-1.md`)

1. **Fewer responses**: FAR doesn't respond to many commands that NXT does. Responses
   for 0x80-0xAF range are mostly absent.

2. **No NXT features**: No `$RED`, `$REE`, `$REF` (BirdMode/RezBoost/TargetAnalyzer) queries.

3. **Different parameter counts**: `$N61,0,0` (2 params) vs NXT `$N61,0,0,0` (3 params)
   — FAR has no screen/dual_range_id.

4. **$N60,1** appears: A DispMode response not seen in NXT capture.

## Commands Not Implemented in Our Rust Code

### Currently Implemented (in `CommandId` enum)

0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x69, 0x6D, 0x6E,
0x77, 0x80, 0x83, 0x84-0x87, 0x89, 0x8A, 0x8D-0x8F, 0x96, 0x9E,
0xAA, 0xAC, 0xD2-0xD5, 0xE0, 0xE3, 0xEA, 0xED-0xEF, 0xF0, 0xFE

### Recommended Additions (by priority)

**High — user-visible features:**

| Hex | Name | Reason |
|-----|------|--------|
| 68 | PulseWidth | Pulse width control, per-range. DRS4D-NXT capture shows it in use. |
| 75 | Tune | Manual/auto tuning control. Present in both captures. |
| A3 | TrailMode | Echo trail settings (8 params). Present in both captures. |
| E1 | TrailProcess | Trail processing on/off. Present in DRS4D-NXT. |
| 88 | RingSuppression | Ring suppression on/off. Queried in both captures. |

**Medium — ARPA/tracking (when we add ARPA support):**

| Hex | Name | Reason |
|-----|------|--------|
| 70 | GuardStatus | Guard zone enable/disable |
| 98 | GuardSelect | Guard zone mode (fan/polygon) |
| 99 | GuardArea1 | Guard zone fan definition |
| 74 | ARPACpaTcpa | CPA/TCPA alarm thresholds |
| D0 | ArpaLostFilterRange | ARPA filter settings |
| BC | ArpaMaxRange | Max ARPA tracking range |

**Low — informational/diagnostic:**

| Hex | Name | Reason |
|-----|------|--------|
| 00 | FreeCommand | Fan status diagnostics. Nice to expose as read-only info. |
| AF | Heartbeat | Already handled implicitly. Could track for connection health. |
| F5 | NN3Command | Hardware telemetry. Read-only diagnostic info. |

### Recommended Fixes to Existing Code

1. **Gain response parsing**: The capture shows the radar echoes Gain changes to
   BOTH screens (`$N63,...,0` and `$N63,...,1`). Our report parser should handle
   the screen parameter and route to the correct RadarInfo.

2. **Sea/Rain response format**: The `<screen>` parameter position varies between
   commands. Our current parser may be ignoring it. Need to verify parameter
   positions against the capture data.

3. **$N83 (MainBang) auto-updates**: The radar auto-adjusts MBS when range changes.
   We should parse these unsolicited `$N83` responses and update the control value,
   rather than only processing them as responses to our own requests.

4. **$NF5 handling**: These NN3 diagnostic messages are very frequent and currently
   cause "unknown command" log entries. Should be silently ignored or logged at
   trace level.

5. **$NAF handling**: The heartbeat `$NAF,256` is very frequent. Should be handled
   at trace level to avoid log noise.
