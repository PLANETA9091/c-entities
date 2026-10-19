# Iteration 3: measurement-integrity fixes + the honest A/B — verdict

Everything before this iteration measured a world without a player. This
document records what was broken, what was fixed, the first honest numbers,
and the architectural conclusion they force.

## What was broken (all found 2026-10-19)

1. **The CI bot never connected.** `bot.js` requires `mineflayer`, which is
   installed into `tools/node_modules` — Node resolves from the script's own
   directory upward and never reaches `tools/`. Every baseline run in the
   whole project (docs 02–07) started with "no player joined within 45s":
   mobs inactive, no push, no AI. Fixed with `NODE_PATH` in
   `run-baseline.sh`. **All previous A/B numbers are invalid as
   "with-player" measurements.**

2. **The CRUSSTY kernel rejected every client handshake** —
   `VerifyError: Bad instruction` at the `tableswitch` in
   `ServerHandshakePacketListenerImpl.handleIntention`. The kernel's
   method-entry transform inserts 3 bytes (bare `invokestatic`), which
   shifts every later switch onto a different 4-byte residue; switch
   padding is aligned against the code-array start, so the padding no
   longer matched and the verifier rejected the whole class. Reproduced on
   a clean runtime with no c-entities module. Fixed in CRUSSTY `49be2c9`:
   entry inserts are always 4 bytes (`invokestatic` + `nop`) — a length
   that is a multiple of 4 preserves every switch's residue and therefore
   its padding, so no re-padding machinery is needed. Regression test
   `method_entry_keeps_switch_padding_valid` (19/19 transform tests green).

3. **The module armed the wrong class copy.** `arm_module` re-resolved
   CeHooks via `find_class`, which can land on a different loader's copy
   than the one `define_class` returned. The woven code saw `ACTIVE=false`
   forever: `ARMED` logged, every query mirrored, probes dead, zero
   exceptions (the catches print — none fired because the flag was simply
   never set on the class that matters). Fixed: `define_bridge` returns a
   global ref of the exact class object; `setActive(true)` goes through it;
   pending JNI exceptions after `call_static_void_method` are checked and
   cleared. Diagnosed by reflective per-scenario counters (diag3: all
   counters 0, `active=false`, mirror traffic live).

4. **Silent kill-switch paths.** Every `catch { ACTIVE=false }` printed
   nothing. Now: `printStackTrace` + `lastErrorText` + mirror-reason
   counters (`nslevel/badWorld/notReady/regrow/desync/disabled`) + a
   `stats()` snapshot, dumped by the bench plugin as a `hooks` block in
   `baseline.json` and periodic `hooks@` log lines.

## First honest A/B (with player, mobs active, 1200-tick window)

| scenario | vanilla mspt | module mspt | delta | fast/mirror | desync |
|---|---:|---:|---:|---|---:|
| items-100k | 88.3 | 91.2 | +3.2% | 5.2k / 0 | 0 |
| mixed-150k | 834.9 | 1374.7 | **+64.7%** | 20.5M / 0 | 0 |
| spread-100k | 377.4 | 552.0 | **+46.2%** | 17.4M / 0 | 0 |

The module is fully live at scale (fast path serves 100% of queries, the
index is in sync — `desync=0`, `notReady=0`) and it **loses** to vanilla.

## Phase breakdown (local diag8, 20k ents, area 150, with player)

Per-query cost of the fast path: **11.8 µs**
- `queryNative` (JNI crossing + batch apply + index query + result copy): **8.4 µs** (71%)
- resolve loop (`sl.getEntity(id)` + `predicate.test` per candidate): **3.1 µs**
- ~11 candidates/query, ~3000 queries/tick → **~35 ms/tick overhead**,
  which is exactly the vanilla→module gap (126 → 161 mspt).

Vanilla's slice scan for the same query: ~0.5 µs (11 candidates × ~20 ns
intersects + predicate on hits).

## Why the architecture cannot win (yet)

- The fast path pays a **fixed ~8.4 µs JNI+native cost per query**; vanilla's
  scan cost is ~20 ns per scanned candidate. The fast path only wins when a
  single query's vanilla scan exceeds ~8 µs (≥ ~600 scanned candidates) —
  rare: vanilla's sectioned storage already bounds scans.
- Per candidate, the fast path pays **resolve + predicate ≈ 210 ns** vs
  vanilla's **intersects ≈ 20 ns + predicate only on hits**. In dense
  regions (most candidates are hits) the fast path is strictly more
  expensive per candidate, plus the fixed cost.
- The index replaces vanilla's 33%-of-CPU intersects scan with
  20%-of-CPU CeHooks work in mixed-150k — but the *other* subsystems then
  dominate, and the net is a large regression.

## Iteration 4 requirements (from this data)

1. **Zero-JNI steady state** — the 8.4 µs/query fixed cost must go below
   ~0.5 µs. Candidates: shared-memory request ring (Java writes query
   boxes via VarHandle, Rust answers into a results buffer), or moving
   the index into Java entirely.
2. **Zero-resolve candidates** — candidates must be usable without an
   id→Entity hash lookup (store references Java-side; resolve cost is
   what makes dense queries unwinnable).
3. Only then can the grid's real edge (no intersects pass, exact XZ/Y
   cells) translate into wins at any density.

The c-entities Rust core, weave patches, kill-switch discipline and the
CRUSSTY injection chain are correct and proven (desync=0 across 38M
queries). What must change is where the per-query boundary sits.
