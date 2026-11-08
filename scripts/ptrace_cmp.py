#!/usr/bin/env python3
"""Compare PTRACE rows of one entity id between the two legs of a scenario."""
import re, sys

LOG = sys.argv[1]
SC = sys.argv[2]          # scenario substring, e.g. "items"
EID = int(sys.argv[3])    # traced entity id (PTRACE rows are for the traced id)
T0, T1 = int(sys.argv[4]), int(sys.argv[5])

lines = open(LOG, encoding="utf-8", errors="replace").read().splitlines()
legs = [(m.group(1), i) for i, ln in enumerate(lines)
        for m in [re.search(r"=== parity leg (parity-\S+) \(module=", ln)] if m]
pt_re = re.compile(r"PTRACE t=(\d+) type=(\w+) tc=(-?\d+) at=(-?\d+) "
                   r'x="([^"]*)" y="([^"]*)" z="([^"]*)" yaw="([^"]*)" '
                   r'vx="([^"]*)" vy="([^"]*)" vz="([^"]*)" onG=(\d+) '
                   r'fire=(-?\d+) alive=(\d+)(?: hp="([^"]*)")?')
F2D = {}
def f2d(h):
    if h not in F2D:
        try:
            F2D[h] = float.fromhex(h)
        except Exception:
            F2D[h] = None
    return F2D[h]

def extract(st, end):
    rows = {}
    for ln in lines[st:end]:
        m = pt_re.search(ln)
        if m:
            rows[int(m.group(1))] = m
    return rows

def leg_range(suffix):
    for k, (name, i) in enumerate(legs):
        if SC in name and name.endswith(suffix):
            end = legs[k + 1][1] if k + 1 < len(legs) else len(lines)
            return i, end
    return None

r1 = leg_range("-off")
r2 = leg_range("-on")
if not r1 or not r2:
    sys.exit("legs not found")
off = extract(*r1)
on = extract(*r2)
print(f"rows: off={len(off)} on={len(on)}")
hdr = None
for t in range(T0, T1 + 1):
    mo, mn = off.get(t), on.get(t)
    if not mo or not mn:
        print(f"{t:>5}  (missing: off={'Y' if mo else 'N'} on={'Y' if mn else 'N'})")
        continue
    if hdr is None:
        hdr = 1
        print(f"{'t':>5} {'field':>4} {'off':>24} {'on':>24}")
    if mo.group(2) != mn.group(2):
        print(f"{t:>5}  TYPE off={mo.group(2)} on={mn.group(2)}")
    for gi, fname in [(3, "tc"), (4, "at"), (5, "x"), (6, "y"), (7, "z"),
                      (9, "vx"), (10, "vy"), (11, "vz"), (12, "onG"),
                      (13, "fire"), (14, "alive"), (15, "hp")]:
        vo, vn = mo.group(gi), mn.group(gi)
        if vo != vn:
            fo, fn = f2d(vo), f2d(vn)
            d = (fn - fo) if (fo is not None and fn is not None) else None
            ds = f"{d:+.6g}" if d is not None else "?"
            print(f"{t:>5} {fname:>4} {vo:>24} {vn:>24}  d={ds}")
