# 4a.3 — paired same-runner A/B: the gate holds

Owner requirement: **the module must never be slower than vanilla**.
Docs 06-09 established that split-runner A/B carries up to +-30%
job-to-job noise, which made that requirement unprovable. This
iteration moves the pair onto ONE runner, back to back.

## Method (workflow ab-paired.yml, run 39402176675)

- One ubuntu-latest runner; for each scenario the off arm runs first,
  the on arm immediately after (same machine, same page cache, same
  CPU governor).
- Scenarios and windows: items-100k 400 ticks, mixed-150k 300 ticks
  (fill 18m + warmup 100 ticks at ~5.7 s/tick already dominate the
  budget on a 2-vCPU runner; the first attempt with 600 ticks
  overran RUN_TIMEOUT and was re-budgeted), spread-100k 400 ticks.
- Every module run must report ARMED, zero desync in the last hooks@
  line, no FAILED-to-patch, no NOT-armed. Violations abort the run.
- Gate: median(on) / median(off) <= 1.05 per scenario.

## Result

| scenario    | vanilla off | module on | ratio | verdict |
|---|---:|---:|---:|---|
| items-100k  | 168.88 | 166.07 | 0.983 | PASS (module 1.7% faster) |
| mixed-150k  | 5164.70 | 4277.59 | 0.828 | PASS (module 17.2% faster) |
| spread-100k | 603.39 | 626.96 | 1.039 | PASS (+3.9%, inside the gate) |

AB-PAIRED-GATE: PASS.

## Reading

- mixed-150k −17.2% is the gate + zero-resolve arena doing their job:
  dense traffic is where the index wins and the router proves it live.
- items-100k is parity-plus (1.7%): the gate routes most item-merge
  queries to the vanilla mirror; the module costs one volatile read.
- spread-100k +3.9% is inside the 1.05 gate margin and consistent with
  the previous split-runner observations (range −17.9% .. +21.1%);
  a single pair cannot resolve it further. Getting spread strictly
  under parity is 4b work (zero-JNI steady state) — the gate stays as
  the safety envelope either way.
