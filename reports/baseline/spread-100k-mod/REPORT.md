# Baseline run: `spread-100k-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88736 -> 88603) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 68.368 / 139.176 / 236.306 / 343.279 |
| MSPT mean, ms | 81.702 |
| TPS estimate (1000/p50) | 14.6 |
| heap used / max, MB | 2029.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8292` |
| 2 | 7.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7538` |
| 3 | 4.51% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 4.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12804` |
| 5 | 3.97% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.09% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.75% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int, int) line: 5585` |
| 8 | 2.53% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 2.28% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5147` |
| 10 | 1.60% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 11 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5326` |
| 13 | 1.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12933` |
| 14 | 1.13% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.11% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |

