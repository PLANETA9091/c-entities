#!/usr/bin/env python3
"""SR/LR first-divergence analyzer for the parity job log.

Pairs the OFF/ON legs of each parity scenario by the `=== parity leg
<name> (module=off|on) ===` markers and compares the per-tick entropy
probes (`SR t=<tick> s=<seed>` for Entity.SHARED_RANDOM, `LR t=<tick>
a=<seed>` for Level.random) plus the per-tick state hash (`PH t=<tick>
h=<hash> n=<count>`). Prints the first divergent tick + stream per
scenario, and the first PH hash/count divergence (the structural state
split) separately from the RNG streams.

Usage: srlr_first_div.py <job-log-file> [scenario-prefix]
"""
import re
import sys

LEG_RE = re.compile(r"^.*=== parity leg (\S+) \(module=(off|on)\) ===")
SR_RE = re.compile(r"SR t=(\d+) s=([0-9a-f]+)")
LR_RE = re.compile(r"LR t=(\d+) a=([0-9a-f]+)")
PH_RE = re.compile(r"PH t=(\d+) ct=(-?\d+) h=([0-9a-f]+) n=(\d+) adds=(-?\d+)")


def main() -> None:
    if len(sys.argv) < 2:
        print(__doc__)
        return
    path = sys.argv[1]
    prefix = sys.argv[2] if len(sys.argv) > 2 else ""
    # data[scenario][leg][tick] = value
    sr: dict = {}
    lr: dict = {}
    ph: dict = {}
    cur = None  # (scenario, leg)
    with open(path, "rb") as f:
        for raw in f:
            line = raw.decode("utf-8", "replace")
            m = LEG_RE.match(line)
            if m:
                scen = m.group(1)
                for suf in ("-off", "-on"):
                    if scen.endswith(suf):
                        scen = scen[: -len(suf)]
                        break
                cur = (scen, m.group(2))
                continue
            if cur is None:
                continue
            scen, leg = cur
            m = SR_RE.search(line)
            if m:
                sr.setdefault(scen, {}).setdefault(leg, {})[int(m.group(1))] = m.group(2)
                continue
            m = LR_RE.search(line)
            if m:
                lr.setdefault(scen, {}).setdefault(leg, {})[int(m.group(1))] = m.group(2)
                continue
            m = PH_RE.search(line)
            if m:
                ph.setdefault(scen, {}).setdefault(leg, {})[int(m.group(1))] = (
                    m.group(3),
                    int(m.group(4)),
                    int(m.group(5)),
                )

    scenarios = sorted(set(sr) | set(lr) | set(ph))
    for scen in scenarios:
        if prefix and not scen.startswith(prefix):
            continue
        verdicts = []
        for name, table in (("SR", sr), ("LR", lr), ("PH", ph)):
            legs = table.get(scen, {})
            off = legs.get("off", {})
            on = legs.get("on", {})
            common = sorted(set(off) & set(on))
            first = None
            for t in common:
                if off[t] != on[t]:
                    first = (t, off[t], on[t])
                    break
            if first is None:
                if not common:
                    verdicts.append(f"{name}: NO COMMON TICKS (off={len(off)} on={len(on)})")
                else:
                    verdicts.append(
                        f"{name}: identical through t={common[-1]} ({len(common)} ticks)"
                    )
            else:
                verdicts.append(
                    f"{name}: FIRST DIVERGENCE t={first[0]} off={first[1]} on={first[2]}"
                )
        print(f"[{scen}] " + " | ".join(verdicts))


if __name__ == "__main__":
    main()
