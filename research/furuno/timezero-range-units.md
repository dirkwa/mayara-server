# TimeZero Range Unit Handling

Decompiled from `Fec.FarApi.dll`, `MaxSea.Radar.dll`, and `MaxSea.SensorApi.dll`.

## Important: Available Ranges Are Hardcoded

**The available range tables are compile-time constants in `Fec.FarApi.dll`.** There is no wire
protocol command to query the radar for its supported ranges. TimeZero determines which ranges
to offer based solely on:

1. The radar model (detected via `RadarSensorTypeIndex`)
2. The active distance unit (NM/SM vs km/Kyd)
3. For DRS6A-NXT: the echo color mode
4. For DRS4D-NXT: the firmware version (>= 1.05 unlocks an extra range)

This means any third-party implementation must replicate these hardcoded tables to know which
ranges are valid for a given radar model and unit combination.

## Unit System

### Wire Protocol Unit Values

The `unit` parameter in `RmcSetRange(radarNo, range, unit)` and the `$S62,<range>,<unit>,<dualRangeId>`
wire command uses these values:

| Value | String | Meaning |
|-------|--------|---------|
| 0     | "NM"   | Nautical Miles |
| 1     | "km"   | Kilometers |
| 2     | "SM"   | Statute Miles (US Miles) |
| 3     | "Kyd"  | Kilo-Yards |

Source: `DistanceUnitString = { "NM", "km", "SM", "Kyd" }` in `Fec.FarApi.dll`.

### TimeZero DistanceUnit Enum

TimeZero's internal `DistanceUnit` enum uses different values:

| Enum | String | Maps to wire |
|------|--------|-------------|
| `NauticalMile` | "NM" | 0 |
| `Kilometer` | "km" | 1 |
| `USMile` | "SM" | 2 |
| *(KiloYard)* | "Kyd" | 3 |

### Unit Grouping for Range Tables

The available range tables group units in pairs:

- **NM (0) and SM (2)**: share the same available range table
- **km (1) and Kyd (3)**: share a different available range table

This makes sense — NM and SM are close in magnitude, as are km and Kyd.

## Setting the Distance Unit

When the user changes the distance unit:

1. TimeZero calls `RmcGetRange(radarNo, out range, out unit)` to get the current range and unit
2. Finds the new unit index by matching against `DistanceUnitString`
3. Calls `RmcSetRange(radarNo, range, newUnit)` — **same range index, different unit**
4. If unit actually changed, also converts the ARPA lost-filter range:
   - Converts the old-unit range to radians (internal reference)
   - Converts from radians to the new unit
   - Calls `RmcSetArpaLostFilterRange` with the converted value

The range index itself does NOT change when switching units. The same native range index
represents different physical distances depending on the active unit.

## Range Value Table

Each column (0-21) maps to a range value via `_rangeSetTbl`, sorted by increasing distance.
The values shown are in the active unit (NM when NM is selected, km when km is selected, etc.):

| Col | Native idx | Value |
|-----|-----------|-------|
| 0   | 21        | 0.0625 |
| 1   | 0         | 0.125 |
| 2   | 1         | 0.25 |
| 3   | 2         | 0.5 |
| 4   | 3         | 0.75 |
| 5   | 4         | 1.0 |
| 6   | 5         | 1.5 |
| 7   | 6         | 2.0 |
| 8   | 7         | 3.0 |
| 9   | 8         | 4.0 |
| 10  | 9         | 6.0 |
| 11  | 10        | 8.0 |
| 12  | 11        | 12.0 |
| 13  | 12        | 16.0 |
| 14  | 13        | 24.0 |
| 15  | 14        | 32.0 |
| 16  | 19        | 36.0 |
| 17  | 15        | 48.0 |
| 18  | 20        | 64.0 |
| 19  | 16        | 72.0 |
| 20  | 17        | 96.0 |
| 21  | 18        | 120.0 |

## Available Ranges Per Radar Model

### RadarSensorTypeIndex (row mapping)

| Index | Radar Model |
|-------|-------------|
| 0     | Unknown |
| 1     | FAR-21x7 |
| 2     | DRS (original) |
| 3     | FAR-14x7 |
| 4     | DRS4DL |
| 5     | FAR-3000 |
| 6     | DRS4D-NXT |
| 7     | DRS6A-NXT |
| 8     | DRS6A X-Class |
| 9     | FAR-15x3 |
| 10    | FAR-14x6 |

### NM/SM Available Range Table

`true` = available. Columns are range values: 0.0625, 0.125, 0.25, 0.5, 0.75, 1, 1.5, 2, 3, 4, 6, 8, 12, 16, 24, 32, 36, 48, 64, 72, 96, 120.

| Model | 0.0625 | 0.125 | 0.25 | 0.5 | 0.75 | 1 | 1.5 | 2 | 3 | 4 | 6 | 8 | 12 | 16 | 24 | 32 | 36 | 48 | 64 | 72 | 96 | 120 | Max |
|-------|--------|-------|------|-----|------|---|-----|---|---|---|---|---|----|----|----|----|----|----|----|----|----|----|-----|
| Unknown       | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | 96 |
| FAR-21x7      | - | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | Y | - | - | Y | - | 96 |
| DRS           | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | 96 |
| FAR-14x7      | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | 96 |
| DRS4DL        | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | 96 |
| FAR-3000      | - | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | Y | - | - | Y | - | 96 |
| **DRS4D-NXT** | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | - | - | - | - | **36** |
| DRS6A-NXT     | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | - | 96 |
| DRS6A X-Class | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | 96 |
| FAR-15x3      | - | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | Y | - | - | Y | - | 96 |
| FAR-14x6      | - | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | Y | - | - | Y | - | 96 |

### km/Kyd Available Range Table

| Model | 0.0625 | 0.125 | 0.25 | 0.5 | 0.75 | 1 | 1.5 | 2 | 3 | 4 | 6 | 8 | 12 | 16 | 24 | 32 | 36 | 48 | 64 | 72 | 96 | 120 | Max |
|-------|--------|-------|------|-----|------|---|-----|---|---|---|---|---|----|----|----|----|----|----|----|----|----|----|-----|
| Unknown       | - | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | 96 |
| FAR-21x7      | - | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | Y | - | - | Y | - | 96 |
| DRS           | - | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | 96 |
| FAR-14x7      | - | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | 96 |
| DRS4DL        | - | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | 96 |
| FAR-3000      | - | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | Y | - | - | Y | - | 96 |
| **DRS4D-NXT** | - | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | - | - | **64** |
| DRS6A-NXT     | - | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | - | 96 |
| DRS6A X-Class | - | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | 96 |
| FAR-15x3      | - | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | Y | - | - | Y | - | 96 |
| FAR-14x6      | - | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | - | Y | - | - | Y | - | 96 |

### Key Observations

**Universal patterns:**
- **0.0625** is only available in NM/SM mode (never in km/Kyd for any model)
- **120** is never available for any model in any unit
- Column 21 (120 NM) is always `false` in the main tables. There is a separate `RemoveDeprecated120NmRange()` in the .NET layer that also strips it for NM mode.

**Model-specific patterns:**

Three distinct range profiles emerge:

1. **Full range (0.125-96)**: DRS, FAR-14x7, DRS4DL, DRS6A X-Class
   - NM/SM: 0.0625 to 96 (21 steps)
   - km/Kyd: 0.125 to 96 (20 steps)

2. **Restricted long range**: FAR-21x7, FAR-3000, FAR-15x3, FAR-14x6
   - Missing: 0.0625, 36, 64, 72 (in both unit modes)
   - Available: 0.125 to 96, skipping 36/64/72 (17 steps in NM/SM, same in km/Kyd)

3. **Short range only**: DRS4D-NXT
   - NM/SM: 0.0625 to 36 (17 steps), 48 with firmware >= 1.05
   - km/Kyd: 0.125 to 64 (18 steps), 48 already included

4. **Medium range**: DRS6A-NXT
   - NM/SM: 0.0625 to 72 (20 steps, missing 96 and 120)
   - km/Kyd: 0.125 to 72 (19 steps)

## DRS6A-NXT Echo Color Mode Override

The DRS6A-NXT has a special case: when echo color mode is active (mode == 1), the available
range table is replaced entirely, regardless of the main table. This applies specifically to
DRS12A-NXT and DRS25A-NXT hardware variants (identified by hostname, not sensor type index).

### DRS6A-NXT Echo Color Range Tables

**NM/SM with echo color (mode 1, standard DRS6A-NXT):**

| 0.0625 | 0.125-24 | 32 | 36-120 |
|--------|----------|----|--------|
| Y      | all Y    | -  | all -  |

Max: **24 NM** (15 steps) — significantly reduced from the normal 72 NM.

**NM/SM with echo color (mode 1, DRS12A-NXT/DRS25A-NXT):**

Same as standard DRS6A-NXT echo color table — max 24 NM.

**km/Kyd with echo color (mode 1, standard DRS6A-NXT):**

| 0.0625 | 0.125-48 | 64-120 |
|--------|----------|--------|
| -      | all Y    | all -  |

Max: **48 km** (17 steps).

**km/Kyd with echo color (mode 1, DRS12A-NXT/DRS25A-NXT):**

Same as standard — max 48 km.

**NM/SM without echo color, DRS12A-NXT/DRS25A-NXT:**

Uses the generic echo color table `ekSAeBTFHq`:

| 0.0625 | 0.125-96 | 120 |
|--------|----------|-----|
| Y      | all Y    | -   |

Max: **96 NM** (21 steps) — full range.

**km/Kyd without echo color, DRS12A-NXT/DRS25A-NXT:**

Uses `VVyA6dYxYQ`:

| 0.0625 | 0.125-120 |
|--------|-----------|
| -      | all Y     |

Max: **120** (21 steps) — the only configuration where 120 is available!

## DRS4D-NXT Firmware Version Check

For DRS4D-NXT with firmware version >= 1.05 (`kgxAELKQxB = 1.05`), column 17 (value 48,
native index 15) is dynamically set to `true` in the active range table at runtime:

```csharp
if (RadarSensorTypeNo(P_0) == RadarSensorTypeIndex.DRS4DNXT
    && _RadarSensorVersion[P_0] >= 1.05)
{
    vVyA6dYxYQ[num5, 17] = true;  // Enable 48 NM/km range
}
```

This extends the DRS4D-NXT from 17 to 18 available ranges in NM/SM mode (adds 48 NM),
and similarly in km/Kyd mode.

## How TimeZero Synchronizes Units

1. **LongDistanceUnit change** (user preference):
   - Propagated via `UnitSettings.PropertyChanged("LongDistanceUnit")`
   - Clears and rebuilds the radar processor
   - Calls `RangeInformation.ChangeUnit(DistanceUnit)` (currently a no-op in the decompiled code)
   - Updates the radar processor's unit

2. **RadarUnit change** (per-radar setting in `T0RadarSettings`):
   - Triggers `RangeInformation.ChangeUnit(DistanceUnit)`
   - Updates radar processor unit
   - Fires `DistanceUnitChanged` event

3. **DistanceUnitA/B sensor** (from `Fec.FarApi`):
   - Read: calls `RmcGetRange(radarNo, out range, out unit)` and returns `DistanceUnitString[unit]`
   - Write: calls `RmcSetRange(radarNo, currentRange, newUnitIndex)` — keeps same range index, changes unit
   - Per dual range: DistanceUnitA for radarNo=0, DistanceUnitB for radarNo=1

4. **Available ranges refresh**: triggered whenever DistanceUnit changes, since different units have
   different available range tables. The `AvailableRangesA`/`AvailableRangesB` sensor properties
   are re-read from the appropriate boolean table.

## Wire Protocol Example

Changing DRS4D-NXT Range A from 6 NM to 6 km:

```
# Current state: range index 9 (=6.0), unit 0 (=NM)
# User changes unit to km

$S62,9,1,0\r\n
# range=9, unit=1(km), dualRangeId=0(Range A)
# Now range index 9 represents 6.0 km instead of 6.0 NM
```

The radar firmware reinterprets the range index according to the unit. The range table
values (0.125, 0.25, 0.5, ...) are not fixed NM — they are in whatever unit is active.
