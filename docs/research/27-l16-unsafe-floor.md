# 27. L16 — the Unsafe floor, the delta mask, and the identity-weak AI table

Date: 2026-10-25. Target: the POST-L15 profile (b0a2095) — the leg is no
longer vanilla work, it is OUR OWN dispatch machinery. The four costs
attacked here were measurable on the CI hardware:

| share (leg self) | method | fix |
|---:|---|---|
| 12.86% | `VarHandleReferences$FieldInstanceReadOnly.get` — every RefField read (6 per tracker guard per entity per tick) | `sun.misc.Unsafe` field offsets captured at RefField build; the read compiles to the getfield itself (~1-2ns vs ~15-30ns) |
| 3.52% | `Entity.setDeltaMovement` — a fresh `Vec3` + isFinite + **synchronized(posLock)** putfield per item per tick, field value identical after landing | `B_DELTA_SAME` mask from the Rust core; the flush skips the call when post-tick delta is bit-identical to pre-tick |
| ~2.0% | `MapMakerInternalMap$Segment.getFirst/getEntry` — the AI floor's per-selector shadow lookup (25k mobs × Guava weak-keys segments) | flat open-addressed identity-weak table (`AIKey` = WeakReference + cached identity hash, tombstone semantics, load-factor rebuilds) |
| 2.17% | `CeHooks.worldId` — the 16-slot scan reached slot 2+ per call with loop bookkeeping | unrolled identity probes for slots 0-2 (the three live benchmark worlds), general scan after |
| ~0.3% | rowQuery `new ArrayList<>(0)` per EMPTY answer (~32k queries/tick) | shared `Collections.emptyList()` (read-only contract; a mutating caller throws and flips the kill switch loudly) |

## Value-exactness of the delta mask

`setDeltaMovement(Vec3)` (javap, 26.3 f649771a...) is exactly:
`v.isFinite()` → `synchronized (posLock) { this.deltaMovement = v; }` —
NO tracker coupling, no data-set, no events. When the core computes
post-tick delta == pre-tick delta bit-exactly, skipping the call leaves
the observable field identical. The resting item reaches that fixed point
mechanically: gravity −0.04 → `move()` collides and zeroes the collided
axis → `0.0 × friction == 0.0` exactly. The core ORs `B_DELTA_SAME` (bit
14) into the out-bits; the flush gates both `setDeltaMovement` and the
`needsSync` path (bit 2 already fires only on real changes).

## The identity-weak table (AI shadows)

Selectors are per-mob — 25k entries, one lookup per selector per tick.
Guava weak-keys map: hash → segment → table chain → WeakReference.get ≈
40ns typical. The replacement: `identityHashCode` → slot → `get()` ≈
5-10ns. Dead slots are tombstones (probes walk past, inserts claim in
place — standard linear-probing rules, safe for every other chain); a
sweep rebuild keeps live pairs when dead entries pile up (mob churn).
Server-thread confined (every caller runs inside the goal phase) — plain
arrays, no fences. `aiShadowPeek` (non-inserting) serves the odd-tick
fast path exactly like the old `AI_SHADOWS.get`.

## Keep-later list

The post-L16 mobs leg still carries the vanilla mob body (aiStep /
travel / navigation / sensing ≈ 70%+ of the leg) and the items leg still
pays the walk + dispatch floor (~10ms). Both are the NEXT structural
moves (collect-at-walk, mob movement floor) — this round only removes
the machinery tax we added ourselves.
