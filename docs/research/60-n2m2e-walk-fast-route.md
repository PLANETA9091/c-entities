# 60 — N2-M2e: the walk fast route (the vanilla lambda dies for natives)

Date: 2026-11-12. Basis: doc 59's post-M2c profile — the mob-attributable
residue was tickListForEach 1.52% + Unsafe.getBoolean 1.01% + the
dispatcher guard chain, ALL of it frames whose bodies have been dead
since M1.5 (the Java tick body), M2a (the dispatcher suppression) and
M2c (the checkDespawn skip) — the WALK kept INVOKING them.

## The route

The L13/L17 walk mirror (`tickListForEach`, the woven replacement of
`EntityTickList.forEach`) already fast-routes native ITEMS straight into
the flush batch. The mob branch was missing: a classified mob still paid
`consumer.accept` → the vanilla lambda (the isRemoved read, the
checkDespawn entry, `tickNonPassenger`) → the M2a guard chain
(`instanceof`, `getPassengers()`, the NmMap probe) — per mob per tick,
every frame inside already-dead code.

The M2e branch sits in BOTH walk bodies (main + the handle-retry twin,
identical shape):

```java
if (mobRoute && e instanceof Mob m
        && m.getPassengers().isEmpty() && nativeMobTick(m)) {
    walkMobNative++;
    continue; // the wire computes this mob at the flush
}
```

- `mobRoute` is computed once per walk: `!MOB_WALK_RAW && ACTIVE &&
  cplugReady && NATIVE_MOB_ON && NATIVE_MOB_HANDLES_OK` — a disarm
  flips every mob back to the vanilla lambda with ZERO bookkeeping
  (no tick-list membership is touched; the L13 mirror keeps the raw-set
  iteration exactly).
- The decision chain is the SAME one the M2a guard runs today
  (`nativeMobTick`: residency, first-sight classification, the
  aware/dead/zombie gates) — one level up. A decline (non-zombie,
  unaware, dying, passenger-carrying vehicle) takes the exact vanilla
  path, whose tick() still fires the sight.
- `-Dcentities.mobWalkRaw=true` refuses the route (the bisect knob,
  mirroring `centities.tickListRaw`).

## Why it is semantics-free for a native

- the lambda's `isRemoved` is a READ — the wire's removal driver (the
  boundary liveness diff) is the remover;
- `checkDespawn` is already M2c-skipped for natives (doc 59's honesty
  ledger: despawn returns with the N3 outbox);
- the dispatcher body (currentlyTicking, the profiler pushes,
  commonTick's setOldPosAndRot/walkDist, checkIfActive, the dead tick(),
  the passengers loop) was already M2a-suppressed — the guard returned
  true on entry; the route just skips the INVOCATION of the guard too;
- the tickCount mirror and the kinematics are the applier's (the M1.5
  contract), the activation semantics were already "the wire always
  ticks" (M1.8b's gate returns active for every native).

The counter proof from the live 10k leg: `dmob=0` — the M2a dispatcher
counter FROZEN at zero across the whole run (the dispatcher is never
entered), while `walk=6.29M` tracks `skip=6.29M` (every native visit
routed at the walk). n=10000 ads=10000 rem=0, jpos bit-exact at every
poll.

## The paired local A/B (this box, same hour, fill300/warm60/measure300, 2 shared vCPU)

| leg | MSPT p50 | mean |
|---|---:|---:|
| vanilla 10k zombies + bot (m10k-van3) | 151.1 | 161.2 |
| **native M2e wire (m10k-m2e)** | **19.4** | 30.5 |

**−87.1% p50.** Against the M2d build on the same box protocol
(21.4, doc 59's addendum) the route reads −9% p50 — the ~2 ms/tick the
dead frames were costing, consistent with the profile's walk+guard mass.
The CI `mobs-25k-native` leg (4-vCPU, back-to-back) stays the −90%
arbiter; the walk residue now scales with CHURN (first-sight
dispatches), not population.
