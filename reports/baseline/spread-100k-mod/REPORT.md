# Baseline run: `spread-100k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88791 -> 88675) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 61.017 / 113.787 / 205.680 / 230.918 |
| MSPT mean, ms | 71.016 |
| TPS estimate (1000/p50) | 16.4 |
| heap used / max, MB | 2390.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8285` |
| 2 | 7.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7533` |
| 3 | 6.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13091` |
| 4 | 4.39% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.86% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.25% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 3.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.37% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 10 | 1.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5216` |
| 11 | 1.53% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.53% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 13 | 1.41% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 1.28% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.16% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

