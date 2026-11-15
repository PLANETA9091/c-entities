# Baseline run: `spread-100k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88731 -> 88608) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 62.112 / 104.354 / 219.498 / 334.573 |
| MSPT mean, ms | 73.426 |
| TPS estimate (1000/p50) | 16.1 |
| heap used / max, MB | 2342.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8335` |
| 2 | 6.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13141` |
| 3 | 5.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7583` |
| 4 | 4.22% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.44% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.33% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.18% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 2.06% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 10 | 1.71% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 11 | 1.65% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 13 | 1.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7454` |
| 15 | 1.08% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |

