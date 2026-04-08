#!/usr/bin/env python3
"""
DRS4W spoke-to-range analysis tool.

Reads a pcap containing Furuno DRS4W UDP spoke traffic on port 10024
and prints per-wire-index statistics about where strong echoes land
in the decoded 430-sample spoke buffer.

Purpose: derive the `wire_index → physical_sample_step` mapping that
makes mayara render targets at the correct radial distance on the
DRS4W. The current code assumes `displayed_range / 430 meters per
sample` which is correct for the DRS4D-NXT but materially wrong for
the DRS4W (targets render ~40% too close to own ship).

Usage:
    tshark -r capture.pcap -Y 'udp.dstport==10024' \\
        -T fields -e frame.time_relative -e udp.payload \\
        > spokes.txt
    ./analyze_pcap.py spokes.txt
"""

import collections
import sys


def decode_enc3(sweep: bytes, sweep_len: int, prev_spoke=None):
    """Decoder for Furuno encoding 3 (delta against previous spoke).
    Mirrors src/lib/brand/furuno/report.rs decode_sweep_encoding_3."""
    if prev_spoke is None:
        prev_spoke = []
    spoke: list[int] = []
    used = 0
    strength = 0
    while len(spoke) < sweep_len and used < len(sweep):
        b = sweep[used]
        if b & 0x03 == 0:
            strength = b
            spoke.append(strength)
        else:
            repeat = b >> 2
            if repeat == 0:
                repeat = 0x40
            if b & 0x01 == 0:
                for _ in range(repeat):
                    i = len(spoke)
                    s = prev_spoke[i] if i < len(prev_spoke) else 0
                    spoke.append(s)
            else:
                for _ in range(repeat):
                    spoke.append(strength)
        used += 1
    return spoke, used


# DRS4W click sequence, observed on Marcel's iPad:
# 0.125, 0.25, 0.5, 0.75, 1, 1.5, 2, 3, 4, 6, 8, 12, 16, 24 NM
WI_TO_NM = {
    0: 0.125, 1: 0.25, 2: 0.5, 3: 0.75, 4: 1.0, 5: 1.5,
    6: 2.0, 7: 3.0, 8: 4.0, 9: 6.0, 10: 8.0, 11: 12.0,
    12: 16.0, 13: 24.0,
}
NM_TO_M = 1852.0
MAIN_BANG_SAMPLES = 20  # samples close to own-ship to ignore as clutter
STRONG_THRESHOLD = 0x40  # raw 6-bit-shifted-left strength value


def analyze(path: str) -> None:
    hists: dict[int, collections.Counter] = collections.defaultdict(collections.Counter)
    n_spokes: dict[int, int] = collections.defaultdict(int)
    prev_spoke: list[int] = []

    with open(path) as f:
        for line in f:
            parts = line.strip().split("\t")
            if len(parts) < 2:
                continue
            try:
                payload = bytes.fromhex(parts[1])
            except ValueError:
                continue
            if len(payload) < 16:
                continue
            b10, b11, b12 = payload[10], payload[11], payload[12]
            sweep_len = ((b11 & 0x07) << 8) | b10
            wire_index = b12 & 0x3F
            sweep_count = payload[9] >> 1
            sweep_data = payload[16:]
            pos = 0
            for _ in range(sweep_count):
                if pos + 4 > len(sweep_data):
                    break
                pos += 4  # per-spoke angle/heading sub-header
                spoke, used = decode_enc3(sweep_data[pos:], sweep_len, prev_spoke)
                n_spokes[wire_index] += 1
                for i, v in enumerate(spoke):
                    if v >= STRONG_THRESHOLD and i >= MAIN_BANG_SAMPLES:
                        hists[wire_index][i] += 1
                used_rounded = (used + 3) & ~3
                pos += used_rounded
                prev_spoke = spoke

    print(
        f"{'wi':>3} {'NM':>6} {'meters':>8} {'spokes':>7}   "
        f"{'far stable':>10} {'far any':>8} {'top peaks (sample, count)':s}"
    )
    print("-" * 100)
    for wi in sorted(hists):
        h = hists[wi]
        nm = WI_TO_NM.get(wi, "?")
        meters = int(nm * NM_TO_M) if isinstance(nm, float) else 0
        stable = [s for s, c in h.items() if c >= 5]
        far_stable = max(stable) if stable else -1
        far_any = max(h.keys()) if h else -1
        top5 = h.most_common(5)
        print(
            f"{wi:>3} {str(nm):>6} {meters:>8} {n_spokes[wi]:>7}   "
            f"{far_stable:>10} {far_any:>8} {top5}"
        )


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(__doc__)
        sys.exit(1)
    analyze(sys.argv[1])
