#!/usr/bin/env python3
"""Merge-scan forensics for the instrumented run: pair MS/MSQ/MSR/QB per scan,
group by (ct, self), and print the cadence/query verdict per scanner."""
import re, sys, collections

LOG = sys.argv[1]
CT0 = int(sys.argv[2]) if len(sys.argv) > 2 else 0
CT1 = int(sys.argv[3]) if len(sys.argv) > 3 else 99999
SELF = sys.argv[4] if len(sys.argv) > 4 else None   # filter: only these self ids (comma-sep)

lines = open(LOG, encoding="utf-8", errors="replace").read().splitlines()
legs = [(m.group(1), i) for i, ln in enumerate(lines)
        for m in [re.search(r"=== parity leg (parity-\S+) \(module=", ln)] if m]
scans = collections.defaultdict(dict)   # (leg, ct, self) -> fields

def leg_of(i):
    cur = None
    for name, st in legs:
        if st <= i:
            cur = name
        else:
            break
    return cur

ms_re = re.compile(r"\[ce-baseline\] MS t=(\d+) self=(\d+) n=(\d+) tc=(-?\d+) mv=(\d) src=(\d)((?: \d+)*)")
msq_re = re.compile(r"\[ce-baseline\] MSQ t=(\d+) self=(\d+) path=(\w+) rawN=(\d+) ids=\[([^\]]*)\] win=(\d+) home=(\d+) stale=(\d+)")
qb_re = re.compile(r"\[ce-baseline\] QB t=(\d+) self=(\d+) q=\[([^\]]*)\] r=([\d.]+)")
msr_re = re.compile(r"\[ce-baseline\] MSR t=(\d+) self=(\d+) (rows|rej)=(\S.*)")

for i, ln in enumerate(lines):
    m = ms_re.search(ln)
    if m:
        key = (leg_of(i), int(m.group(1)), int(m.group(2)))
        scans[key]["ms"] = m
        continue
    m = msq_re.search(ln)
    if m:
        key = (leg_of(i), int(m.group(1)), int(m.group(2)))
        scans[key]["msq"] = m
        continue
    m = qb_re.search(ln)
    if m:
        key = (leg_of(i), int(m.group(1)), int(m.group(2)))
        scans[key]["qb"] = m
        continue
    m = msr_re.search(ln)
    if m:
        key = (leg_of(i), int(m.group(1)), int(m.group(2)))
        scans[key].setdefault(m.group(3), []).append(m.group(4))

selfs = set(int(x) for x in SELF.split(",")) if SELF else None
n_shown = 0
for (leg, ct, self_id), d in sorted(scans.items()):
    if not (CT0 <= ct <= CT1):
        continue
    if selfs and self_id not in selfs:
        continue
    ms = d.get("ms")
    msq = d.get("msq")
    parts = [f"{leg} ct={ct} self={self_id}"]
    if ms:
        parts.append(f"n={ms.group(3)} tc={ms.group(4)} mv={ms.group(5)} src={ms.group(6)} cands={ms.group(7).strip() or '-'}")
    if msq:
        parts.append(f"path={msq.group(3)} rawN={msq.group(4)} ids=[{msq.group(5)}] win={msq.group(6)} home={msq.group(7)} stale={msq.group(8)}")
    if "qb" in d:
        parts.append("q=[" + d["qb"].group(3) + "] r=" + d["qb"].group(4))
    if "rej" in d:
        parts.append("rej=" + "; ".join(d["rej"][:2]))
    if "rows" in d:
        parts.append("rows=" + d["rows"][0][:90])
    print("  ".join(parts))
    n_shown += 1
    if n_shown > 400:
        print("... (truncated)")
        break
if n_shown == 0:
    print("(no scans matched)")
