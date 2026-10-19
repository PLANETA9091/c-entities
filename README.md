# c-entities

Entity & block-entity tick optimization for Minecraft, built as a native
Rust module for the [CRUSSTY](https://github.com/PLANETA9091/CRUSSTY)
platform (JVM injection via bytecode weaving).

**Goal**: tick **all** entities in **≤ 0.5 ms** with 100k+ entities and
100k+ block entities on the map, rock-solid 20 TPS, no increase in RAM/CPU
usage — and **100% vanilla behavior for players**. Base server:
**Paper 1.21.8**, Java 21.

## Phases (top to bottom — no skipping)

| Phase | What | Status |
|-------|------|--------|
| 0 | Understand the Paper 1.21.8 tick pipeline from real sources | ✅ `docs/research/01-vanilla-tick-pipeline.md` |
| 1 | **Baseline benchmark of unmodified Paper** on GitHub Actions: what costs what | ✅ 9 scenarios, `reports/baseline/SUMMARY.md` |
| 2 | Design the Rust core from phase-1 data (not guesses) | ✅ `centities-core` (exact XZ grid, property-tested) |
| 3 | `centities-module` (weave injection) boots on real Paper 1.21.8 via CRUSSTY | ✅ e2e green: all patches applied by the JVM, module ARMED |
| 4 | A/B on the same pipeline: Paper vs Paper+module | 🚧 iteration 1 done — honest result: module slower in dense scenarios, analysis + iteration-2 plan in `docs/research/06-ab-iteration1.md` |

No optimization code is written before phase 1 produces numbers.

## Benchmark pipeline

Each `baseline.yml` matrix job boots on its own runner:

1. Paper 1.21.8 (official Fill API), superflat world, noise turned off.
2. A fake player (mineflayer, creative) — without a player Paper's Entity
   Activation Range sends everything into `inactiveTick()` and the
   measurement would lie.
3. The `ce-baseline` measurement plugin: plugin chunk tickets (level 31),
   gradual fill from the run matrix, MSPT recorded via
   `ServerTickEndEvent`, results dumped to `baseline.json`.
4. Java Flight Recording → top hot methods → `hotspots.md`
   ("what loads what").
5. A summary is committed back to `reports/baseline/` by the publish job.

Matrix: control / items 25k–100k / mobs 10k–25k / mixed 50k–150k /
tiles 25k–100k (hoppers + furnaces).

## Layout

```
docs/research/     — vanilla pipeline analysis, methodology, injection points
baseline/plugin/   — measurement-only Paper plugin (Maven)
scripts/           — fetch/build/run/report tooling (CI-only)
reports/baseline/  — committed by CI after every run
```
