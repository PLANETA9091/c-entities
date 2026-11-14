# Baseline run: `spread-100k-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88787 -> 88649) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 59.564 / 113.973 / 203.203 / 245.452 |
| MSPT mean, ms | 69.969 |
| TPS estimate (1000/p50) | 16.8 |
| heap used / max, MB | 2540.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7942` |
| 2 | 8.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7189` |
| 3 | 6.74% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12574` |
| 4 | 4.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.95% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 3.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.44% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 5297` |
| 8 | 3.19% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 2.63% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4897` |
| 10 | 2.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.53% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 12 | 1.51% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.44% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.40% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 1.12% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |

