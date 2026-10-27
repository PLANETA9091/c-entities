# 23. L12 — the native push kernel (the 80% lever)

Date: 2026-10-24. Directive: "мега ускорение, всё на -90%, масштабные
изменения на Rust, поведение как в ваниле". Everything below is measured
(ab-paired #12 artifacts) or javap-verified against the shipped
paper-26.3-218 jar.

## 1. Where the time actually went (ab-paired #12, WITH the bench bot)

The doc-21 verdict table was the first honest playered measurement, and it
left items at parity (0.969) and mixed at 0.808. The fresh JFR of run
40031698683 (mixed-150k, module leg, 160k samples) answers why:

| share of ALL samples | chain |
|---:|---|
| **79.95%** | `LivingEntity.pushEntities()` → `Level.getPushableEntities` → `CeHooks.getEntities` → `CeHooks.mirror` → `EntityLookup.getEntities` → `ChunkEntitySlices$EntityCollectionBySection.getEntities` → `AABB.intersects` |
| 56.92% | leaf = `AABB.intersects(DDDDDD)` alone |

Every mob's push is a section scan costing O(section population), and the
bench's anchor section holds ~96k items + 30k stands + the pile. Both the
mirror AND the native index scan are O(population) there — the EMA gate
honestly prefers the mirror, and the AVX2 scan cannot change the asymptotic.
The pile zombie's push list is ~7.5k survivors from ~140k candidates; the
vanilla push loop is O(N²) **by behavior** (every zombie queries every
neighbor every tick), so the win must come from a cheaper O(N²), not from
skipping it.

## 2. The kernel (L12)

`crates/centities-module/src/pushcore.rs` computes one
`LivingEntity.pushEntities()` natively per pusher, synchronously in the
weave guard (intra-tick causality preserved: later pushers and the victim's
own `travel` see every force the same tick vanilla would). Components:

* **Pushable-only index (`pidx`)** — a second `EntityIndex` fed by the same
  probe batches with a per-entry flag byte. Push candidates stop paying for
  the 130k non-pushable items/stands sharing the cell.
* **The push mirror (`PushCore`)** — per-id: exact f64 box6, exact x/z,
  delta xyz, flag bits, direct vehicle id, `numCollisions`, plus the
  **kernel_add ledger** (below). Slot = `id & MASK` with the full id stored
  (removals clear the key synchronously — a removed entity can never be a
  candidate).
* **The force loop** — the exact 26.3 bytecode (`Entity.push`,
  `pushEntities`, javap dumps verified): candidate served in **entity-id
  order** (vanilla's per-section candidate order is insertion order ≈ id
  order for the farm populations; the core's `sort_cell_by_id` re-sorts a
  cell at most once per (cell, tick)), exact f64 `AABB.intersects(AABB)`
  strict compares, `absMax` metric, `0.009999999776482582` eps,
  `0.05000000074505806` quantum, `f = min(1/d, 1)`, mutual delta writes,
  `needsSync`.
* **numCollisions** — javap-verified: the ONLY reader/writer outside
  `pushEntities` itself is the constructor. The mirror is authoritative;
  the loop increments it BEFORE `doPush` (an `Entity.push` eps return never
  undoes it — caught by the unit tests), the reset `max(0, ncol - maxEC)`
  runs before the loop, and the `maxEntityCollisions` cap `return`s the
  whole body exactly like the bytecode.
* **The cramming audit** — `list.size() > cramming-1` counts ALL exact-test
  survivors (the cap does not stop the counting — vanilla counts the whole
  list); the `random.nextInt(4)` roll happens in Java, in exactly the
  vanilla short-circuit condition, on the entity's own `RandomSource`
  (MethodHandle — the field is protected), then `hurtServer(cramming, 6f)`.
* **Events** — `EntityCollideWithEntityEvent` listener presence routes the
  push vanilla (the bench registers none); `push(DDD, null)` never fires
  `EntityPushedByEntityAttackEvent` (source is null) so no listener check
  is needed there.

### The delta merge scheme (two writers, one truth)

The mirror has two writers: Java (vanilla knockback/explosions/plugins,
via `probeDelta`/`probeBox` events) and the kernel (pair forces). A naive
"last write wins" lets a queued Java event **erase pending kernel pushes**.
The mirror therefore keeps a per-slot ledger:

```
mirror = java_value + kernel_add          (invariant)
Java event V (always the PRE-kernel Java field):
    mirror = V + kernel_add               (rebase; pushes survive)
Kernel pair write f:
    mirror += f ; kernel_add += f
Self writeback (the guard's setDeltaMovement(mirror)):
    kernel_add = 0                        (Java field == mirror now)
```

Every Java delta change funnels through `setDeltaMovement`/`setPos` →
`probeDelta`/`probeBox` (both probed), so the invariant holds by
construction. `numCollisions` needs no ledger: only `pushEntities` touches
it, and the kernel owns that body.

## 3. What Java still does per kernel-owned pusher

The guard (`CeHooks.pushDispatch`, woven as an EarlyReturnGuard on
`LivingEntity.pushEntities()`): ~10 field/config reads + one JNI crossing;
then the self writeback (`setDeltaMovement` — a fresh Vec3 like vanilla,
`needsSync = true`, the `numCollisions` MethodHandle setter) and the
cramming branch. Everything else — the O(section) query, the O(candidates)
predicate application, the per-pair `doPush` Java floor — is native.

## 4. The guard's vanilla-routing contract

false (vanilla body runs) when: not living / ServerPlayer / client or
non-ServerLevel / removed / noPhysics / **passenger** (riders keep vanilla;
their push semantics are rarely exercised and never in the bench) /
not-pushable (the byte-faithful `isPushable` port: `isAlwaysTicking`,
`isPositionEntityTicking`, alive, !spectator, `collides`, !onClimbable) /
any team on self / cramming<=0 && maxEC<=0 / event listeners registered /
world unknown / kernel mirror miss. The kernel-declined path re-syncs the
mirror's `numCollisions` into the Java field first, so the vanilla body
starts from the right counter.

## 5. Documented deviation envelope (same discipline as merge order)

* **Candidate order across cell boundaries**: per-cell id-sorted; a push
  box spanning cells processes cells in (cx, cz) order — vanilla orders by
  section-y layers then insertion. For the packed pile (one cell) the
  order is bit-identical; movers across boundaries may see a different
  `maxEntityCollisions` neighbor selection. Same deviation class the
  project already accepts for merge order (proven order-independent
  equilibrium) — here it changes WHICH neighbors a capped pusher picks.
* **Rare flag staleness**: pushable flips without a box event (a villager
  falling asleep, a spectator flip) re-classify on the entity's next box
  probe. Behaviorally inert: the affected entities do not run `travel`,
  so their mirrored delta never integrates into position; `numCollisions`
  on a dying entity is unread (its own pushEntities is gated by its state).
* **Deferred velocity packets**: entities pushed AFTER their own tick this
  tick get their Java `deltaMovement` at their next kernel call. Server
  positions are exact; the tracker's velocity packet for exactly those
  entities can lag one tick (inactive-but-tracked boundary ring).
* **Oversize pushables** (half-span > 8 cells — none in vanilla mobs) are
  not kernel candidates; they route vanilla through the guard's mirror
  miss.

## 6. Tests

* 13 `pushcore` unit tests: the force math replicates the exact vanilla op
  sequence (`dx /= d` division, NOT `dx * (1/d)` — one ULP apart, caught),
  eps-return keeps `numCollisions` (the vanilla loop order), the cap picks
  the lowest ids after the sort, the cramming audit ignores the cap, the
  merge scheme preserves pending pushes across Java events, removed/same-
  vehicle candidates behave byte-faithfully.
* `natives_parity` extended: `nativeFlush` `(I[J[D[I[D[D[II[J[D)V`,
  `nativePush` `(IIIIIIII)J`, `nativePushNcol` `(I)I` — the Java/Rust
  descriptor drift fails in `cargo test`, not in a CI boot (it caught a
  `[J` vs `[I` typo in the registration during development).
* The full core suite (24 tests) stays green with `sort_cell_by_id` /
  `query_cell` added.

## 7. Harness

`ab-paired` gains **mobs-25k** (the scenario the matrix never measured);
e2e asserts the new patch line. Kill switches: `-Dcentities.pushNative=false`
(the whole kernel), the existing `centities.mergeFast` / `centities.rowPath`
untouched.

## 8. Expected verdict (to be measured by the next paired A/B)

mixed-150k: the 79.95% mirror-push share becomes the native kernel at
~2-4ns/pair vs vanilla's ~36ns — targeting module ≈ 600-750ms vs vanilla
2546 (-70..-76%). mobs-25k: the 12.5k-zombie anchor pile is the same shape
(12.5k² pairs) — the kernel is the first lever that attacks it at all.
items-100k: untouched by this lever (its floor is the tracker/sync stack —
the next lever, L13).
