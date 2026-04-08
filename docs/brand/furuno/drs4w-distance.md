# DRS4W spoke-to-range open question

## Problem

On the Furuno DRS4W (Wi-Fi), mayara renders targets at the wrong radial
distance. With the current `furuno-improvements` PR they plot at roughly
40% of their true range: a target visible at the 3 NM ring on the iPad
app lands near the 0.5 NM ring in mayara at the same display scale.

The DRS4D-NXT uses the same code path and renders correctly. The DRS4W
is the only model with this regression.

## What we know

1. **Wire format matches NXT for everything except distance.** The UDP
   frame header at port 10024 is identical: `packet_type=0x02`, the same
   spoke sub-headers, `encoding=3`, `sweep_len=430`, angle/heading in
   the per-spoke 4-byte sub-header. Decoding produces clean output with
   no byte-accounting issues.
2. **`wire_index` walks 0..13 on the DRS4W** in the same order as the
   iPad click sequence (`0.125, 0.25, 0.5, 0.75, 1, 1.5, 2, 3, 4, 6, 8,
   12, 16, 24` NM). The current `WIRE_INDEX_TABLE` (NM mode) mapping
   for wire indices 0..13 matches those NM values one-for-one.
3. **All other header fields are constant across ranges on the DRS4W**:
   `byte 13 = 0x00`, `byte 14 = 0xf0`, `byte 15 lower bits = 0x08`.
   There is no per-range field we are ignoring.
4. **`sweep_len` is always 430** regardless of range setting.

## What we have tried

### Treat 430 as the real sample count (`src/lib/brand/furuno/report.rs`
`stretch_spoke`)

We assume the 430 decoded samples cover the full configured range and
stretch them to `FURUNO_SPOKE_LEN=1024` (the GUI's per-angle slot size).
Targets still render ~40% too close.

This is the same formula that works on DRS4D-NXT where the native
`sweep_len` already equals 1024. On the DRS4W the same arithmetic
over-scales by roughly a factor of 2.5.

## Evidence from `radar12.pcap` (issue #48)

Marcel captured a walk through every range setting at the dock on
2026-04-09. `tools/drs4w/analyze_pcap.py` decodes each frame and
histograms the sample positions that carry strong returns
(`strength >= 0x40`), ignoring the first 20 samples as main-bang /
own-ship clutter.

Per-range "farthest stable strong return" (sample index in the 0..429
decoded spoke; "stable" = at least five hits across the capture):

| wire_index | iPad scale | NM table meters | farthest stable sample |
|------------|-----------:|---------------:|------------------------:|
| 0          | 0.125 NM   |            231 |                      40 |
| 1          | 0.25 NM    |            463 |                     130 |
| 2          | 0.5 NM     |            926 |                      71 |
| 3          | 0.75 NM    |           1389 |                      50 |
| 4          | 1.0 NM     |           1852 |                      38 |
| 8          | 4.0 NM     |           7408 |                      54 |
| 10         | 8.0 NM     |          14816 |                      34 |

These numbers are too inconsistent to fit a clean
`samples_per_meter(wire_index)` function. The farthest **any** strong
sample goes up to 264 at wi=0 and 68 at wi=8 — suggesting the radar's
physical bin step and/or pulse width changes with range, as TimeZero's
`RadarRanges` → pulse-width tables imply (S1/S2/M1/M2/M3/L).

We do not yet have a reliable model for the DRS4W that explains all
seven data points above.

## What we still need

A capture where the radar is pointed at a **single, known-distance
reflector** (e.g. a buoy whose GPS position is measured, or a fixed
object at a surveyed range from the boat). With the object's physical
distance known, each wire_index gives us exactly one data point:

    step_meters_at(wire_index) = known_distance_meters / sample_index_of_peak

Three such data points at widely separated ranges (e.g. 0.5 NM, 2 NM,
8 NM) would be enough to fit (or confirm) the sample-step function and
replace `stretch_spoke`'s linear assumption with the real bin-to-range
mapping.

## Analysis tool

`tools/drs4w/analyze_pcap.py` reproduces the histogramming above. To
use:

```sh
tshark -r capture.pcap -Y 'udp.dstport==10024' \
    -T fields -e frame.time_relative -e udp.payload > spokes.txt
python3 tools/drs4w/analyze_pcap.py spokes.txt
```

## Tracking

- Original issue: #48
- Follow-up branch: `furuno-drs4w` (this branch)
- Partial fix currently on `main` via `furuno-improvements`: linear
  stretch from 430 → 1024 samples, which fixed the "image confined to
  inner 49% of screen" symptom but left the remaining ~40% under-plot.
