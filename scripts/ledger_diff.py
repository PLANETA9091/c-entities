#!/usr/bin/env python3
"""ledger_diff.py <parity-joblog> [victim_ids] [--window lo-hi] — the L101
MLEDGER analyzer.

Pairs the OFF/ON legs of each parity scenario (the `=== parity leg
parity-<scen>-<mod> (module=..)` markers) and:

  * reconstructs the ON leg's per-mob SERVE chains from the MLEDGER
    records (a fresh call rs=0 opens a serve; chained records follow;
    st=0 ST_DONE closes it). The s1->s0 gaps quantify each staged
    predicate's draws;
  * diffs the PTRACE lines (tg=/goalSelector=/aw=/x=/yaw=) for the
    victim ids at every tick both legs printed;
  * names the first tick a victim's serve STARTED a goal the OFF leg's
    PTRACE selector state contradicts (the wrong-resume fingerprint).

Usage: ledger_diff.py joblog.txt 107,139 --window 154-164
"""
import re
import sys
from collections import defaultdict

LEG_RE = re.compile(r"=== parity leg parity-(\w+)-(off|on) \(module=")
ML_RE = re.compile(
    r"MLEDGER t=(\d+) seq=(\d+) id=(\d+) tc=(\d+) s0=([0-9a-f]+) "
    r"s1=([0-9a-f]+) dr=(\d+) st=(\d+) rs=(\d+) k=([0-9a-f]+)")
PTR_RE = re.compile(r"PTRACE t=(\d+) (.*)$")
PTR_FIELD = re.compile(r"(\w+)=([^ ]*)")
SEL_RE = re.compile(r"(targetSelector|goalSelector)=\[(.*?)\]")


def legs(lines):
    """split the joblog into ordered (scen, mod, [lines]) sections"""
    out = []
    cur = None
    for ln in lines:
        m = LEG_RE.search(ln)
        if m:
            cur = (m.group(1), m.group(2))
            out.append((cur, []))
        elif out:
            out[-1][1].append(ln)
    return out


def parse_mledger(text):
    """tick -> list of records; record = dict"""
    per_tick = defaultdict(list)
    for m in ML_RE.finditer(text):
        per_tick[int(m.group(1))].append({
            "t": int(m.group(1)), "seq": int(m.group(2)),
            "id": int(m.group(3)), "tc": int(m.group(4)),
            "s0": int(m.group(5), 16), "s1": int(m.group(6), 16),
            "dr": int(m.group(7)), "st": int(m.group(8)),
            "rs": int(m.group(9)), "k": int(m.group(10), 16),
        })
    return per_tick


def parse_ptrace(text):
    """tick -> field dict (the PTRACE line is the single traced mob; the
    line body keeps t= type= tc= at= nc= aw= tg= x= ... + the selectors)"""
    out = {}
    for m in PTR_RE.finditer(text):
        tick = int(m.group(1))
        body = m.group(2)
        fields = dict(PTR_FIELD.findall(body))
        fields["_body"] = body
        out[tick] = fields
    return out


def serves(recs, want_ids):
    """group records into per-id chains: rs==0 opens, st==0 closes"""
    chains = defaultdict(list)
    for r in sorted(recs, key=lambda r: r["seq"]):
        if r["id"] not in want_ids:
            continue
        chains[r["id"]].append(r)
    return chains


def chain_report(ch):
    lines = []
    serve_no = 0
    open_serve = None
    for r in ch:
        fresh = (r["rs"] == 0)
        if fresh:
            serve_no += 1
            open_serve = []
        if open_serve is not None:
            open_serve.append(r)
        if r["st"] == 0 and open_serve is not None:
            s0 = open_serve[0]["s0"]
            s1 = open_serve[-1]["s1"]
            total_dr = sum(x["dr"] for x in open_serve)
            gaps = [open_serve[i + 1]["s0"] - open_serve[i]["s1"]
                    for i in range(len(open_serve) - 1)]
            kinds = 0
            for x in open_serve:
                kinds |= x["k"]
            kinds_str = ",".join(str(b) for b in range(1, 9) if kinds >> b & 1) or "-"
            lines.append(
                f"  serve#{serve_no} t={open_serve[0]['t']} tc={r['tc']} "
                f"rounds={len(open_serve)} draws={total_dr} "
                f"kinds={kinds_str} s0={s0:016x} s1={s1:016x} "
                f"gaps={[g for g in gaps if g != 0] or '0'}")
            open_serve = None
    return lines


def ptrace_summary(fields):
    """compact the PTRACE body for the diff"""
    keep = {}
    for sel in ("targetSelector", "goalSelector"):
        m = re.search(re.escape(sel) + r"=\[(.*?)\]", fields.get("_body", ""))
        if m:
            keep[sel] = m.group(1).strip()
    for k in ("tg", "aw", "x", "yaw", "vx"):
        if k in fields:
            keep[k] = fields[k]
    return keep


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return 1
    path = sys.argv[1]
    ids = set()
    win = None
    argv = sys.argv[2:]
    if argv and argv[0].replace(",", "").isdigit():
        ids = {int(x) for x in argv[0].split(",")}
        argv = argv[1:]
    for i, a in enumerate(argv):
        if a == "--window" and i + 1 < len(argv):
            lo, hi = argv[i + 1].split("-")
            win = (int(lo), int(hi))
    text = open(path, encoding="utf-8", errors="replace").read()
    lines = text.splitlines()

    for (scen, mod), sec in legs(lines):
        if mod != "on":
            continue
        ml = parse_mledger("\n".join(sec))
        all_ids = {r["id"] for rs in ml.values() for r in rs}
        targets = ids or all_ids
        if not all_ids:
            continue
        print(f"=== {scen} ON: {sum(len(v) for v in ml.values())} records, "
              f"{len(all_ids)} served mobs ===")
        for rid in sorted(targets):
            ch = [r for t in sorted(ml) for r in ml[t] if r["id"] == rid]
            if not ch:
                continue
            print(f"mob {rid}:")
            for ln in chain_report(ch):
                print(ln)

    # PTRACE pairing per scenario (off vs on) for the victim ids
    secs = {}
    for (scen, mod), sec in legs(lines):
        secs.setdefault(scen, {})[mod] = "\n".join(sec)
    for scen, pair in secs.items():
        if "off" not in pair or "on" not in pair:
            continue
        off = parse_ptrace(pair["off"])
        on = parse_ptrace(pair["on"])
        ticks = sorted(set(off) | set(on))
        for rid in sorted(ids):
            diver = []
            for t in ticks:
                fo = off.get(t)
                fn = on.get(t)
                if fo is None and fn is None:
                    continue
                so = ptrace_summary(fo or {})
                sn = ptrace_summary(fn or {})
                sel_o = so.get("goalSelector", "")
                sel_n = sn.get("goalSelector", "")
                if fo is None or fn is None or sel_o != sel_n or \
                        so.get("tg") != sn.get("tg") or so.get("x") != sn.get("x"):
                    diver.append((t, sel_o, so.get("tg"), sel_n, sn.get("tg")))
            if diver:
                print(f"--- {scen} traced mob {rid or '?'}: PTRACE/selector divergence at "
                      f"{len(diver)} ticks (first {min(t for t, *_ in diver)}) ---")
                for t, so, tgo, sn, tgn in diver[:6]:
                    print(f"  t={t}")
                    print(f"    OFF sel=[{so}] tg={tgo}")
                    print(f"    ON  sel=[{sn}] tg={tgn}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
