#!/usr/bin/env python3
"""Extract MS/MSQ/MSR merge-scan telemetry for a leg at a ct range (CUR_TICK space)."""
import re, sys

LOG = "/tmp/parity_fail.log"
LEG = sys.argv[1]          # e.g. parity-items-on
CT0, CT1 = int(sys.argv[2]), int(sys.argv[3])
PAT = sys.argv[4] if len(sys.argv) > 4 else "MS"

lines = open(LOG, encoding="utf-8", errors="replace").read().splitlines()
legs = [(m.group(1), i) for i, ln in enumerate(lines)
        for m in [re.search(r"=== parity leg (\S+) \(module=", ln)] if m]
idx = {}
for j, (n, i) in enumerate(legs):
    idx[n] = (i, legs[j + 1][1] if j + 1 < len(legs) else len(lines))

st, end = idx[LEG]
rx = re.compile(r"\[ce-baseline\] (" + PAT + r") t=(\d+)\b(.*)")
out = 0
for ln in lines[st:end]:
    m = rx.search(ln)
    if m:
        ct = int(m.group(2))
        if CT0 <= ct <= CT1:
            # strip the STDOUT wrapper prefix if present
            body = re.sub(r"^.*?Z \[[^\]]*\](?: \[[^\]]*\])?: ", "", ln)
            print(body)
            out += 1
if out == 0:
    print(f"(no {PAT} lines in {LEG} ct={CT0}..{CT1})")
