# Baseline run: `spread-100k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88725 -> 88570) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 61.116 / 109.410 / 206.715 / 288.843 |
| MSPT mean, ms | 72.289 |
| TPS estimate (1000/p50) | 16.4 |
| heap used / max, MB | 2525.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7583` |
| 2 | 4.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8335` |
| 3 | 4.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13141` |
| 4 | 3.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 3.67% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.38% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 3.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7410` |
| 9 | 2.53% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 2.47% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 11 | 1.66% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 12 | 1.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 13 | 1.56% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 14 | 1.42% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.30% | `net.minecraft.world.entity.Entity.getType() line: 633` |

