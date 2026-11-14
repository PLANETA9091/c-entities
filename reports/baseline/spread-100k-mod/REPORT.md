# Baseline run: `spread-100k-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88760 -> 88642) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 65.352 / 124.612 / 228.604 / 331.787 |
| MSPT mean, ms | 77.747 |
| TPS estimate (1000/p50) | 15.3 |
| heap used / max, MB | 2261.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8008` |
| 2 | 6.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7255` |
| 3 | 5.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12640` |
| 4 | 4.95% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 4.01% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 2.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.49% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 2.39% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5320` |
| 9 | 2.35% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4904` |
| 10 | 2.12% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 11 | 1.77% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.64% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7195` |
| 14 | 1.27% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 15 | 1.14% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |

