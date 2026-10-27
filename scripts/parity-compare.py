#!/usr/bin/env python3
"""Behavioral parity gate: compare two entity-state snapshots (module off/on).

Snapshots are produced by BaselinePlugin (PARITY=1): every entity's full
observable state with EXACT bit patterns (hex floats/doubles), sorted by
entity id. Two servers that behave identically must produce identical
snapshots.

Verdict levels per field:
  - bitwise equality is the default requirement (same JVM, same hardware,
    same inputs -> identical arithmetic; any divergence is a REAL behavior
    difference somewhere);
  - --tol D relaxes positional/velocity/yaw fields to |delta| <= D (per
    snapshot, not cumulative) for debugging FP-order drift — the CI gate
    runs at tol=0.

Exit code 0 = PARITY PASS, 1 = FAIL, 2 = structural error (missing files,
empty snapshots, id/type mismatches).
"""
import json
import math
import sys


def parse_hex(s):
    if s is None:
        return None
    return float.fromhex(s)


def load(path):
    with open(path) as f:
        return json.load(f)


def main():
    if len(sys.argv) < 3:
        print("usage: parity-compare.py <off.json> <on.json> [--tol D] [--max-report N]")
        return 2
    off_path, on_path = sys.argv[1], sys.argv[2]
    tol = 0.0
    max_report = 12
    args = sys.argv[3:]
    i = 0
    while i < len(args):
        if args[i] == "--tol":
            tol = float(args[i + 1]); i += 2
        elif args[i] == "--max-report":
            max_report = int(args[i + 1]); i += 2
        else:
            i += 1

    a, b = load(off_path), load(on_path)
    fails = []

    if a.get("tag") != b.get("tag"):
        fails.append(f"tag mismatch: {a.get('tag')} vs {b.get('tag')} (snapshot ticks differ — legs desynced)")
    if a.get("seed") != b.get("seed"):
        fails.append(f"seed mismatch: {a.get('seed')} vs {b.get('seed')}")

    ea = {e["id"]: e for e in a["entities"]}
    eb = {e["id"]: e for e in b["entities"]}

    if a["n"] != b["n"]:
        fails.append(f"entity count: off={a['n']} on={b['n']}")
    only_off = sorted(set(ea) - set(eb))
    only_on = sorted(set(eb) - set(ea))
    if only_off:
        fails.append(f"{len(only_off)} entities only in off: ids {only_off[:max_report]}")
    if only_on:
        fails.append(f"{len(only_on)} entities only in on: ids {only_on[:max_report]}")

    type_mismatch = 0
    worst = {}

    def track(field, delta):
        if field not in worst or abs(delta) > abs(worst[field]):
            worst[field] = delta

    exact_fields = ["onG", "fire", "alive", "hurtT", "amt", "pd"]
    float_fields = {
        "x": 1.0, "y": 1.0, "z": 1.0,
        "yaw": 1e-4, "pitch": 1e-4,
        "vx": 1e-9, "vy": 1e-9, "vz": 1e-9,
        "hp": 0.0,
    }

    drifted = []
    for eid in sorted(set(ea) & set(eb)):
        x, y = ea[eid], eb[eid]
        if x["t"] != y["t"]:
            type_mismatch += 1
            if type_mismatch <= max_report:
                fails.append(f"id {eid}: type {x['t']} vs {y['t']}")
            continue
        for f in exact_fields:
            if f in x or f in y:
                if x.get(f) != y.get(f):
                    drifted.append((eid, f, x.get(f), y.get(f), None))
        for f, scale in float_fields.items():
            if f in x or f in y:
                va, vb = parse_hex(x.get(f)), parse_hex(y.get(f))
                if va is None or vb is None:
                    drifted.append((eid, f, x.get(f), y.get(f), None))
                    continue
                d = va - vb
                track(f, d)
                limit = tol * scale if tol > 0 else 0.0
                if not (abs(d) <= limit or (va == vb) or (math.isnan(va) and math.isnan(vb))):
                    drifted.append((eid, f, va, vb, d))

    print(f"parity compare: {off_path} vs {on_path}")
    print(f"  tag={a.get('tag')} seed={a.get('seed')} n_off={a['n']} n_on={b['n']}")
    print(f"  common ids: {len(set(ea) & set(eb))}, only-off: {len(only_off)}, only-on: {len(only_on)}, type mismatches: {type_mismatch}")
    if worst:
        print("  worst deltas (abs):")
        for f, d in sorted(worst.items(), key=lambda kv: -abs(kv[1])):
            print(f"    {f:>6}: {d!r}")
    if drifted:
        fails.append(f"{len(drifted)} drifted field values (showing up to {max_report}):")
        for eid, f, va, vb, d in drifted[:max_report]:
            fails.append(f"  id {eid} {f}: off={va!r} on={vb!r} delta={d!r}")

    if fails:
        print("  PARITY-GATE: FAIL")
        for f in fails:
            print("    " + f)
        return 1
    print("  PARITY-GATE: PASS (bit-identical state)" if tol == 0 else f"  PARITY-GATE: PASS (within tol={tol})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
