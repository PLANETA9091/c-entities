# 58 — N2-M1.8b/M2: the activation gate + the dispatcher floor + the three live-boot wire fixes

Date: 2026-11-11 (sandbox 2 vCPU; the CI 4-vCPU mobs-25k-native legs are the
−90% arbiter). Wave on top of doc 57 (the wholesale wire, the first
production A/B −71.6% p50 @10k).

## The forensics that started the wave

The first local 10k zombie leg after the M1.7 commit booted with the module
ARMED but the wholesale wire DEAD: `mob=0/0/0`, the fill zombies ticked
vanilla. Two independent boot bugs, both silent:

1. **M1.7's liveness field was looked up on the wrong class.**
   `O_ENT_DEAD = offOf(Entity.class, "dead")` — javap 26.3: `Entity` has NO
   `dead` boolean (the removal signals are `removalReason` / `pluginRemoved`
   / `valid`). The field lives on **`LivingEntity.dead`** (set by `die()` the
   instant health hits 0). The miss resolved −1 → `initNativeMobHandles`
   declined → `NATIVE_MOB_HANDLES_OK=false` → the whole wire refused, and
   the only log line was "mob AI floor handle probe failed" with no reason.
   Fix: `LivingEntity.class` + the probe now logs the throwable.
2. **The zombie-family weave chained the burnUndead probe onto Zombie
   bytes.** `weave_burn_probe` inserts a call at the head of
   `burnUndead()V` — javap 26.3: the method is **Mob-private**; Zombie does
   not declare it (it inherits). `insert_static_call_at_start` failed,
   `PATCHES_FAIL++`, and the arm gate (`fail==0`) killed the arm on every
   `CE_NATIVE_MOB=1` boot ("NOT armed (ok=16, fail=2)"). Fix: the burn hunt
   rides the Mob weave only.
3. **(infra) the mapped jar without a sibling `libraries/` tree starved
   javac** — the build.rs classpath walk looks for `libraries/` next to the
   jar's ancestor dirs; `build/paperboot` had none → 871 "package
   net.minecraft does not exist" errors. Fix: `CE_MAPPED_JAR` override +
   `cargo` sourced from the rustup env inside the leg script.

## M1.8b — the activation gate (and its dead first build)

`ActivationRange.checkIfActive(Entity)` runs per entity per tick and walks
`isAlive() → getHealth() → the FULL SynchedEntityData chain` for every mob
past tickCount 200 (the 10k JFR: ~2.6% of the server thread — 83% of the
getHealth slice). For an in-store native the answer is irrelevant (both the
tick and inactiveTick routes die in the Mob.tick guard).

The weave: op-4 early-return guard at the head of `checkIfActive`
(`nativeMobTracked(Entity)Z` — a pure NmMap probe, no classification, no
counters; the sight stays pinned to the tick route). **The first build
shipped DEAD**: op-4's receiver heuristic emitted `ALOAD 1` — correct for an
instance method (local 0 = this), a VerifyError for a STATIC method (no
local 1 exists) — and `checkIfActive` is static. The retransform was
silently rejected and the gate had no counter, so nothing ever noticed; the
m10k-native7 profile still showed the getHealth walk. Fix in CRUSSTY
cplug-sdk (`ce-m2-static-fix`): statics pass their first parameter as
local 0. After the rebuild: "precomputed activation gate (native
fast-active)" appears and the SynchedEntityData slice drops to 0.5%.

## M2a — the dispatcher floor dies for natives

The vanilla dispatcher (`ServerLevel.tickNonPassenger`, javap 26.3) per
entity per tick: currentlyTicking bookkeeping, TWO profiler pushes (one
invokedynamic lambda alloc), `commonTick` (setOldPosAndRot/walkDist),
`checkIfActive`, then `tick()`/`inactiveTick()` + postTick + the passengers
loop. For a native mob every one of these is behavior-free: the Java body
is dead (M1.5), the tickCount mirror stays with the applier, and
`checkDespawn` lives in the WALK lambda (outside this dispatcher) — despawn
stays exact.

Weave: the existing L7 `itemDispatch` guard gains the native-mob branch —
`NATIVE_MOB_ON && ent instanceof Mob m && m.getPassengers().isEmpty() &&
nativeMobTick(m)` → the whole dispatcher is suppressed. Passengers route
vanilla (a suppressed vehicle would starve its riders — the item guard uses
the same gate). Unclassified mobs take the vanilla dispatch, whose tick()
fires the sight. Live proof: `dmob=5,864,958` in the N2 poll line (10k mobs
× ~586 boundaries — every native mob every tick).

## M2b — the applier's direct position read

The read-back compare (`px != m.getX() || py != m.getY() || pz != m.getZ()`)
was 60% of the Entity.get[XYZ] slice (~1.9% of the thread at 10k): three
virtual calls per mob per boundary. Now: `U.getObject(m, O_ENT_POS)` (the
private `position` Vec3) + three plain `getDouble`s on the public final
`Vec3.x/y/z`. The offsets are handle-gated — a miss keeps the whole wire
off.

## Infra: the console parasite

The sandbox pty gave jline a LIVE terminal: the console redisplay loop
(LineReaderImpl.redisplay + AttributedString churn + EnumMap.clear +
getKeys/getVariable) ate ~6% of the JFR samples and stole CPU from the
server thread. The bench server's stdin now rides `/dev/null` (both legs —
paired valid; CI runners were already dumb terminals).

## The paired numbers on this box (2 vCPU, noisy, ±40%)

| leg | p50 ms/tick | mean | p90 |
|---|---:|---:|---:|
| vanilla 10k zombies + bot (m10k-van2) | 199.9 | 230.3 | 363.7 |
| native wire (m10k-native7, pre-M2) | 51.9 | 67.8 | 149.5 |
| native wire (m10k-native8, M2 live) | 54.6 | 68.8 | 136.8 |

**−74.0% p50** (199.9 → 51.9). The M2 deltas (~4-5% of the thread
combined) are inside the sandbox noise — the mechanisms are verified by the
live counters (dmob, the activation-gate marker, the SynchedEntityData
slice), and the `mobs-25k-native` CI leg (dedicated 4-vCPU runners,
back-to-back A/B) is the arbiter for the −90% line.

## The fresh profile shape (m10k-native8, 3937 samples)

Mob-path residue after M2: the walk floor (tickListForEach 1.4-3.5% —
shared), checkDespawn ~1.0% (despawn semantics — kept exact on purpose),
the applier's Unsafe reads ~3% (the M2b leaf), the tracker ~1%,
SynchedEntityData 0.5%. The boot/infra noise (paperclip bzip2, watchdog,
jline pre-fix) ≈ 19% of samples — not tick work. The wire itself is lean;
the remaining tick wall is chunk/world/packet work shared with vanilla.

## Next

- The CI arbiter legs (mobs-25k-native) — the −90% verdict at 25k/4 vCPU.
- M3 (the N3 event outbox: damage/sounds/drops — returns environmental
  death; the natives are currently immune to lava/fire because baseTick is
  dead by design).
- The GC pauses (~3.7% of the leg wall, young collections 40ms avg on the
  noisy box) — paired-valid; revisit only if the CI legs demand it.
