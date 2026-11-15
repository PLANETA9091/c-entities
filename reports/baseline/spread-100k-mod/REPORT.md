# Baseline run: `spread-100k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88709 -> 88585) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 41.265 / 63.596 / 167.327 / 234.563 |
| MSPT mean, ms | 48.850 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 2464.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.11% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 5.83% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13091` |
| 3 | 5.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 4.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7533` |
| 5 | 4.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8285` |
| 6 | 3.85% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 7 | 2.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.28% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5220` |
| 10 | 1.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12990` |
| 11 | 1.37% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 12 | 1.36% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 0.91% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5216` |
| 15 | 0.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7422` |

