# Baseline run: `spread-100k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88815 -> 88691) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 60.199 / 116.687 / 208.383 / 247.267 |
| MSPT mean, ms | 71.354 |
| TPS estimate (1000/p50) | 16.6 |
| heap used / max, MB | 2397.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8321` |
| 2 | 7.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7569` |
| 3 | 6.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13127` |
| 4 | 4.29% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.58% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 2.16% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 10 | 2.09% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 11 | 1.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.50% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 13 | 1.43% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 1.34% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.08% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

