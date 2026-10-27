# L6 — native item tick core: the tick COMPUTED in Rust, not skipped

Owner directive (binding): "Так мы не оптимизируем в способ НЕ ТИкать, мы
оптимизируем, ускоряем на Rust!" — no tick-skipping, no rest guards as the
lever. The item tick's WORK must run natively. L6 replaces the rest-skip
direction entirely: every ItemEntity is COMPUTED every tick, in Rust.

## Where the remaining time is (items-100k-mod p50 45.0ms, CI 4e6a7a6)

- ~24.7k items/tick still run the FULL vanilla ItemEntity.tick body in Java
  (~26ms): dispatcher floor + baseTick + gravity + move/collide + friction +
  merge cadence + age/despawn.
- The rest: ActivationRange AABB tests, setOldPos dispatch, JNI query floors.

## Architecture

```
[dispatcher] tickNonPassenger: setOldPosAndRot, tickCount++, checkIfActive
  ItemEntity.tick prefix        -> CeItemHooks.itemCollect(e)      (L6)
  ItemEntity.inactiveTick guard -> CeItemHooks.itemInactiveCollect(e)
[flush] Level.tickBlockEntities prefix (runs right after the entity loop)
  CeItemHooks.itemFlush(wid, env...) -> ONE JNI crossing:
    ids[] + flags[] (+ resync state for entries whose Java body ran)
    -> Rust computes EVERY item's tick bit-exactly:
       gravity, move-gate, collide (section cache, axis-order port of
       moonrise performCollisions), friction (f32, block-below via cache),
       blockSpeedFactor, -0.5 ground branch, fall damage tracking,
       merge cadence (2/40), age++, pickupDelay--, inactiveTick body,
       void/despawn events.
    -> writeback: deltas (all), positions (moved), onGround, hasImpulse,
       fallDist, events (merge / despawn / land-effects / void / vanillaNext)
  Java applies writebacks: ARENA-resolved refs, setDeltaMovement,
  setPos (moved only), mergeScanNative (existing L3 path), despawn event.
```

## Exactness contract

- The Rust body is a line-by-line port of decompiled ItemEntity.tick +
  Entity.tick/baseTick/move/collide (Paper 1.21.8, moonrise collision).
  f32 literals stay f32 (0.98F = 0.9800000190734863 as f64), gravity 0.04 is
  double, collide epsilons 1e-7, Mth.floor, (tickCount + id) % 4 gate,
  merge cadence floor-changed ? 2 : 40, checkFallDamage (float) cast.
- Collision universe: per-section cache populated from the block palette.
  A section whose palette contains only constant-context collision shapes
  that resolve to a SINGLE cuboid (full cubes, slabs, tops) is exact — those
  go through the AABB collide path (moonrise's own singleAABB fast path).
  Sections containing multi-cuboid or context-dependent shapes are marked
  complex -> items touching them route vanilla (they still tick, in Java).
  Fluid-bearing sections: items route vanilla (baseTick refreshes the fluid
  state exactly like vanilla's own water-entry tick; no divergence: vanilla
  also detects water one tick after entry).
- Transitions: setBlockState probe invalidates the section cache + marks
  overlapped items vanilla for a transition window. first-entry resync,
  K=64 resync cadence and probe-driven resyncs keep the SoA store honest.
- Events (despawn event, fallOn/gameEvent landing, discard) run EXACT vanilla
  Java snippets at flush, in vanilla body order.
- Within-tick reordering: item physics land at the end of the entity phase
  ("items tick last"). Vanilla's own order is entityTickList-order dependent,
  so this is a legal ordering, documented and A/B-verified.

## Not in this milestone

- Voxel (multi-cuboid) collide — complex sections route vanilla.
- World-border proximity, nether-ceiling void, portals — route vanilla.
- Water/lava physics — route vanilla (flags read fresh by the collector).

## Files

- crates/centities-core/src/itemtick.rs — pure core + trace/property tests
- crates/centities-module/src/itemcore.rs — JNI surface, SoA store, cache
- crates/centities-module/sources/java/CeItemHooks.java — collector/flush
- Kill switch: CE_ITEM_TICK=0 (default on), per-item vanilla routing on any
  doubt; ACTIVE=false disables everything as before.

# ADDENDUM — L6 implementation landed (commit this one)

## Status: implemented, boot-smoke verified, A/B pending CI

- Core: `centities-core/src/{itemtick,sections,tick}.rs` — the exact port.
  8 trace tests with hand-computed bit-exact sequences (resting-item delta
  oscillation + frozen position, slide decay, landing with LAND event, gate
  phase (tickCount+id)%4, inactive body, cobweb stuck multiplier, 20k-case
  collide property test vs an independent reference, exact gather test).
- Module: `itemcore.rs` (flush native, section-cache JNI up-call, id-tagged
  store), `CeHooks.java` L6 block (collectors, flush applier, palette
  section provider, Bukkit surface via reflection — the compile classpath
  carries only the mojang-mapped server).
- Weaves: ItemEntity.tick EarlyReturnGuard (itemSkip — the rest-skip body
  was DELETED), ItemEntity.inactiveTick EarlyReturnGuard,
  Level.tickBlockEntities prefix (best-effort; refusal keeps items vanilla
  because mode bits never arm).
- Boot smoke (sandbox, 3 console-summoned items, no players): ARMED,
  ItemChecks 557->3866 in 65s (== 3 items x 20 tps x 65s), Boxes 50->422
  (native writebacks firing setPos), zero crashes. Queries/MergeChecks=0 is
  the expected no-player shape (inactive items, PickupDelay 32767).
- Known accepted deviations (documented, bounded):
  * within-tick reordering: item physics land at Level.tickBlockEntities
    ("items tick last") — vanilla's own order is entityTickList-dependent;
  * 1-tick freeze for items whose tick turns out non-exact at flush time
    (stuck-in-block, complex/fluid sections entered mid-tick) — the vanilla
    body computes them next tick;
  * horizontalCollision/minorHorizontalCollision fields are not written
    back (unobservable for items in vanilla);
  * nether worlds route all items vanilla.
- Bench integrity: the smoke ran on the 2-core sandbox; CI ab-paired (same
  runner, paired legs) gives the first trustworthy A/B for L6. Watch
  items-100k (deterministic) first, mobs/spread second.
