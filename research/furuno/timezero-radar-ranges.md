# TimeZero Radar Range Analysis

Decompiled from `MaxSea.Radar.dll` and `MaxSea.SensorApi.dll` (TimeZero/Furuno).

## RadarRanges Enum

Defined in `MaxSea.SensorApi.dll` as `RadarRanges : uint`. The enum index (0-21) is what gets
passed in sweep data as a `short`. The `RangeValue` is the distance in the configured unit
(NM by default). `NumberOfIntervalInNn3Product` controls ring interval count.

| Index | Enum           | Range (NM) | Ring Intervals | Ring Spacing (NM) |
|-------|----------------|------------|----------------|--------------------|
| 0     | R_000_0625     | 0.0625     | 2              | 0.03125            |
| 1     | R_000_125      | 0.125      | 2              | 0.0625             |
| 2     | R_000_25       | 0.25       | 2              | 0.125              |
| 3     | R_000_5        | 0.5        | 2              | 0.25               |
| 4     | R_000_75       | 0.75       | 3              | 0.25               |
| 5     | R_001_0        | 1.0        | 4              | 0.25               |
| 6     | R_001_5        | 1.5        | 3              | 0.5                |
| 7     | R_002          | 2.0        | 4              | 0.5                |
| 8     | R_003          | 3.0        | 3              | 1.0                |
| 9     | R_004          | 4.0        | 4              | 1.0                |
| 10    | R_006          | 6.0        | 3              | 2.0                |
| 11    | R_008          | 8.0        | 4              | 2.0                |
| 12    | R_012          | 12.0       | 4              | 3.0                |
| 13    | R_016          | 16.0       | 4              | 4.0                |
| 14    | R_024          | 24.0       | 4              | 6.0                |
| 15    | R_032          | 32.0       | 4              | 8.0                |
| 16    | R_036          | 36.0       | 3              | 12.0               |
| 17    | R_048          | 48.0       | 4              | 12.0               |
| 18    | R_064          | 64.0       | 4              | 16.0               |
| 19    | R_072          | 72.0       | 4              | 18.0               |
| 20    | R_096          | 96.0       | 4              | 24.0               |
| 21    | R_120          | 120.0      | 4              | 30.0               |

## Key Details

### Range unit
Configurable via `DistanceUnit`: NauticalMile, Kilometer, or KiloYard.

### 120 NM range deprecated
`RemoveDeprecated120NmRange()` filters R_120 from available ranges when unit is NM.

### Dual range
`DrsRange` enum has `Range1` and `Range2` — the radar can display two simultaneous ranges.
Dual range capability is checked via `RadarCapability.DualRange`.

### Sweep format
```
CreateSweep(radarNo: short, status: short, echo: byte[], sweep_len: short,
            scale: short, range: short, angle: short, heading: short, hdg_flag: bool)
```
- `range` is the enum index (0-21), not the actual distance
- 2048 spokes per rotation (derived from lost echo accounting code)
- Default initialization uses `RadarRanges.R_006` (6 NM)

### Pulse width vs range (DRS / DRS4DL / DRS X-Class, NM unit)

| Pulse Width | Min Range Index | Max Range Index |
|-------------|-----------------|-----------------|
| S1          | 0               | 6  (1.5 NM)    |
| S2          | 3 (0.5 NM)     | 7  (2.0 NM)    |
| M1          | 5 (1.0 NM)     | 9  (4.0 NM)    |
| M2          | 7 (2.0 NM)     | 11 (8.0 NM)    |
| M3          | 8 (3.0 NM)     | 14 (24.0 NM)   |
| L           | 10 (6.0 NM)    | (max)           |

### Pulse width vs range (FAR21x7 / FAR15x3, NM unit)

| Pulse Width | Min Range Index | Max Range Index |
|-------------|-----------------|-----------------|
| S1          | 0               | 7  (2.0 NM)    |
| S2          | 3 (0.5 NM)     | 9  (4.0 NM)    |
| M1          | 4 (0.75 NM)    | 11 (8.0 NM)    |
| M2          | 8 (3.0 NM)     | 14 (24.0 NM)   |
| M3          | 8 (3.0 NM)     | 14 (24.0 NM)   |

### Pulse width vs range (DRS / DRS4DL / DRS X-Class, Km/KiloYard unit)

| Pulse Width | Min Range Index | Max Range Index |
|-------------|-----------------|-----------------|
| S1          | 0               | 8  (3.0)        |
| S2          | 4 (0.75)        | 10 (6.0)        |
| M1          | 6 (1.5)         | 11 (8.0)        |
| M2          | 9 (4.0)         | 13 (16.0)       |
| M3          | 11 (8.0)        | 17 (48.0)       |
| L           | 12 (12.0)       | (max)           |

### Range-to-PulseWidth setting keys
Each range index maps to a per-range pulse width setting key (`PulseWidth0` through `PulseWidth21`),
allowing per-range pulse width configuration stored in settings.
