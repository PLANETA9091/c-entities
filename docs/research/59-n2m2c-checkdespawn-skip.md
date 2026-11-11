# doc 59 — N2-M2c: the checkDespawn residency skip + the paired 10k production pair

Branch `main` @ post-M2c (01e4b54). Doc 58 closed the dispatcher floor
(M2a) and the applier read-back compare (M2b). This wave closes the last
per-mob per-tick VANILLA walk the wire still paid: `Mob.checkDespawn` —
called from the entity-tick lambda for every mob every tick, OUTSIDE the
dispatcher the M2a guard covers.

## The cost (the 10k JFR, pre-M2c)

`checkDespawn` for a resident native ran the FULL vanilla body:

- `despawnRanges.get(getType().getCategory())` — the Paper config map
  lookup per mob per tick — the profile's `HashMap.getNode` leaf chain
  (64% of 2.9% of ALL-thread samples ≈ 1.9%),
- the `level.paperConfig()` walk (`entities.spawning.despawnRanges`
  + `despawnRangeShape` + `despawnRate` reads),
- `findNearbyPlayer(mob, -1.0, PLAYER_AFFECTS_SPAWNING)` (0.39%),
- `getDifficulty()` / `isPersistenceRequired()` /
  `requiresCustomPersistence()` (0.3%).

Sum: ~3% of the server thread at 10k. All of it dead weight for a mob
whose population is store-driven.

## The weave

The FIFTH op-4 guard on the same Mob blob (chained after tick / aiStep /
serverAiStep in `precompute_mob`):

```
checkDespawn()V <- CeHooks.nativeMobTracked(Entity)Z
```

`nativeMobTracked` is the PURE residency probe (the activation gate's
check — instanceof Mob + the primitive NmMap `containsKey`); no
classification, the sight stays pinned to the tick route. The receiver
(Mob) is assignable to the check's Entity parameter — the same
link-safety shape as the Zombie.tick weave (exact descriptor == link
safety, the Paper-26.3 entity-discard lesson).

## The honesty ledger

- A resident native no longer hard-despawns — the Paper despawn ranges
  (the 128-block instant-despawn for monsters, the soft-rate range)
  return with the N3 event outbox. The wire's removal driver still covers
  death/discard exactly (`Entity.dead` from die() + `isRemoved()` for
  discard/chunk-unload — the M1.7 predicate).
- The bench population is persistenceRequired (the fill spawns helmeted
  persistent zombies) — despawn never fired for it in ANY leg, so the
  bench numbers are unchanged by definition.
- Non-native mobs: the guard declines (containsKey misses), vanilla
  checkDespawn byte-identical. CI parity/e2e untouched (the whole
  natives-only guard chain is gated on CE_NATIVE_MOB=1 without
  centities.mobAi).

## The paired 10k production A/B (same window, same box hour)

zvan4 / znat12 — 10000 persistent helmeted zombies, flat world, the
mineflayer bot (ViaVersion), fill 300 + warmup 60 + measure 300, 2 shared
vCPU, Xmx1536M, the module at 01e4b54 (M2a dispatcher floor + M2b
read-back compare + M2c despawn skip + the stdin/console fix):

| leg | MSPT p50 | mean | TPS est. |
|---|---:|---:|---:|
| vanilla (module off) | 283.4 | 313.5 | 3.5 |
| **native M2c wire** | **47.9** | 60.9 | **20.0** |

**−83.1% mspt p50 / −80.6% mean at 10k, player-present, full duty.**
The wire state: `n=10000 ads=10000 rem=0` across the whole run, zero
churn, jpos bit-exact at every poll, the dispatcher counter
`dmob=4.97M` (the M2a floor riding every native mob-tick).

Session line: the first production A/B (doc 57, pre-M1.5b) was −71.6%
p50; M1.5b/M1.7 + M2a/M2b + the console fix + M2c took the same-box pair
to −83.1%. The box is 2 shared vCPU with ±40% run-to-run noise — the
`mobs-25k-native` CI leg (dedicated 4-vCPU runners, back-to-back A/B)
remains the −90% arbiter; the executor line (doc 55) already projects
par(4) 0.53–0.80 ms = −90.5…−93.7% in-bench.

## The post-M2c profile (what is left on the server thread)

The mob-attributable leaves at 10k, 3680 samples:

| leaf | % all samples | note |
|---|---:|---|
| tickListForEach (the walk) | 1.52 | the entity-loop floor itself |
| Unsafe.getDouble | 1.96 | the applier/dispatch primitive reads |
| Unsafe.getBoolean | 1.01 | the liveness + guard checks |
| ttlCachedPath | 0.92 | the tracker gate (L72, already fill-cached) |
| setPosRaw | 0.49 | the walking population's vanilla setPos |
| DataItem.getValue | 0.48 | the tracker's dirty scan (real players) |

Everything mob-attributable sums to ~7% of all-thread samples; the rest
is the harness (WatchdogThread 10%, bzip2 4%) and the shared server floor
(chunk scheduling, worldgen tail, GC). The Java-side mob floor is closed:
what remains is the boundary walk, the applier's primitive writes, the
tracker's sync duty and the vanilla setPos — all load-bearing.

## The dead-end ledger

- The activation-gate weave (M1.8b first build) shipped DEAD: the op-4
  helper emitted ALOAD 1 into the STATIC `checkIfActive` frame — the
  retransform was rejected with JVMTI_ERROR_FAILS_VERIFICATION, silently
  (no counter). Fixed two ways: the sdk op-4 learns STATIC methods
  (CRUSSTY d7ae117 — local 0 is the first parameter for a static frame)
  AND the M2a dispatcher floor made the weave redundant for mobs (the
  dispatcher dies before checkIfActive). The weave stays woven (it is
  correct after the sdk fix) but its mob-side value is zero.
- The zombie-family arm-time retransform (M1.8a): the deferred verifier
  path can never fire for pre-arm-loaded classes — `precompute_zombie_family`
  REMOVES the bytes from the stash, so the deferred `STASH.contains_key`
  gate is dead; the guard is only real if the class retransform rides the
  ARM loop (ZOMBIE_CLASS + ZOMBIE_VILLAGER_CLASS added). The m10k-native8
  profile caught it: ConversionTracker.tick still rode Zombie.tick (the
  M1.5b override-hole fix was a no-op on pre-arm boots).
- A local 25k vanilla leg does not fit the sandbox budget (worldgen +
  fill at 2 shared vCPU exceed the 600s tool-call ceiling) — the 25k
  production pair belongs to the CI runners.

## Addendum — M2d: the console dies; the paired 10k pair reads −87.4% p50

The stdin=/dev/null fix (38fa93e) silenced the jline pty redisplay loop but
the PumpThread still SPUN on the instant EOF (~1.5% of the samples — the
reader never parks). The bench launcher now passes `--noconsole`
(org.bukkit.craftbukkit.Main parses it in 26.3): the console reader thread
is never created. Both A/B legs share the flag — the pairing stays valid.

The re-run pair (same window fill300/warm60/measure300, same hour, both
legs --noconsole, the disk also un-saturated — the bench server trees were
pinned at 250MB per run and the rootfs had hit 100%):

| leg | MSPT p50 | mean |
|---|---:|---:|
| vanilla (module off) | 169.7 | 188.4 |
| **native M2d wire** | **21.4** | 33.4 |

**−87.4% mspt p50 at 10k, player-present, full duty** (the wire: n=10000
ads=10000 rem=0, dmob live, jpos bit-exact). The console reader had been
stealing CPU slices from BOTH legs on the 2-vCPU box — the vanilla leg
more (its tick is CPU-saturated; 283.4 → 169.7 with the flag). The mob-
attributable native cost is now nearly invisible against the shared server
floor (chunk system, GC, network); the remaining distance to −90% belongs
to the CI 4-vCPU runners (the arbiter), where the executor's parallel arm
also scales (doc 55: par(4) −90.5…−93.7% in-bench).

Ops note for the next wave: the rootfs hit 100% — every bench server dir
pins ~250MB (the paperclip libraries tree). The cleanup that unblocked
this pair: `rm -rf bench/*/server` (results kept; the runner recreates the
tree per leg).
