# 22 — This branch vs mainline (Paper 26.3)

This branch (`module-v02-1.21.8`) is a complete, ARMED, A/B-measured
implementation of the exact entity index for **Paper 1.21.8**. It was built
in parallel with the mainline's 26.3 retarget; mainline remains the living
trunk. Numbers here (mobs-25k −37.3 %, mixed-150k −38.4 %, 2080 differential
checks / 0 mismatches) were produced by `scripts/run_ab.sh` on a live server
without a bench bot.

Ideas demonstrated here that are portable to mainline (26.3):

1. **Pure-Rust Level body swap** (`patch::replace_body_with_static`) — a
   straight-line bridge body needs no StackMapTable at all, removing the
   ASM/`defineClass` dependency inside ClassFileLoadHook entirely. Root
   cause it sidesteps: `defineClass` during a retransform deadlocks a
   stopped world (class-loader lock held by a safepointed thread).
2. **ARM ordering**: wait `MinecraftServer.isReady()` → retransform probes
   (no-ops while `ACTIVE=false`) → bootstrap the index from
   `EntityLookup.getAll()` → flip ACTIVE → retransform Level. No stale
   window, no mid-boot retransform deadlock.
3. **Cost-aware fast path**: the native reply carries `[candidates,
   visited]`; a population pre-estimate (`query_population`, O(cells))
   returns a `-1` marker for huge dense queries before the walk. Wide
   sensor boxes in dense packs mirror vanilla instead of losing on
   Java-side id resolution (mobs-25k: 11.8 M fast / 3.7 M fallback).
4. **Bounds-checked prefix patcher**: exception table lives at `code_end`
   (not +8), nested attributes are variable-length, LVT start-0 entries grow
   their length instead of shifting, verification_info tags are 0–6 empty /
   7,8 = +2 bytes, switch refusal uses a real opcode-length scan (generated
   `OP_LEN` table) — the naive byte scan false-refused `removeEntity`.
5. **4th weave** `Level.getEntitiesOfClass(Class,AABB,Predicate)` — a
   general fallback merge/sensor coverage on 1.21.8 (mainline covers merge
   via its own `MERGE_ON` path instead).
6. **CE_DIFF harness mode**: paired vanilla-vs-index id-set comparison in
   the benchmark warmup window; telemetry (probes, dedup hits, fallbacks)
   lands in `baseline.json`.

Not portable as-is: everything mapped to 1.21.8 descriptors/classes
(mojang-mapped names differ in 26.3), and the 16-block AoS cell grid here —
mainline's blocked-SoA AVX2 scan supersedes it on 26.3.
