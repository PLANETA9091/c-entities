# Baseline run: `spread-100k-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88786 -> 88647) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 64.606 / 133.501 / 234.674 / 324.937 |
| MSPT mean, ms | 77.160 |
| TPS estimate (1000/p50) | 15.5 |
| heap used / max, MB | 2235.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8160` |
| 2 | 6.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7407` |
| 3 | 5.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12966` |
| 4 | 4.24% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 4.00% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.02% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.91% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5413` |
| 8 | 2.28% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 2.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 2.11% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 11 | 1.96% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4981` |
| 12 | 1.45% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.31% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7347` |

