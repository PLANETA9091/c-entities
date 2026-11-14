# Baseline run: `spread-100k-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88721 -> 88584) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 62.393 / 119.000 / 226.379 / 319.637 |
| MSPT mean, ms | 73.910 |
| TPS estimate (1000/p50) | 16.0 |
| heap used / max, MB | 2164.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8224` |
| 2 | 6.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7471` |
| 3 | 5.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13030` |
| 4 | 4.19% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 2.87% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.72% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 2.32% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.24% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 10 | 1.82% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4986` |
| 11 | 1.62% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7411` |
| 13 | 1.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5199` |
| 14 | 1.20% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 1.13% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |

