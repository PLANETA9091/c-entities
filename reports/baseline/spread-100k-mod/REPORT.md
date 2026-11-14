# Baseline run: `spread-100k-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88844 -> 88717) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 63.700 / 125.412 / 230.341 / 314.799 |
| MSPT mean, ms | 76.203 |
| TPS estimate (1000/p50) | 15.7 |
| heap used / max, MB | 2479.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8218` |
| 2 | 6.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13024` |
| 3 | 5.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7465` |
| 4 | 4.87% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.60% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 2.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.11% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 10 | 1.49% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4986` |
| 11 | 1.47% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5199` |
| 13 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7405` |
| 14 | 1.22% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 15 | 1.16% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |

