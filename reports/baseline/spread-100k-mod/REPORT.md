# Baseline run: `spread-100k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88795 -> 88656) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 64.171 / 129.313 / 237.177 / 351.982 |
| MSPT mean, ms | 77.271 |
| TPS estimate (1000/p50) | 15.6 |
| heap used / max, MB | 2111.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8299` |
| 2 | 6.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13105` |
| 3 | 5.61% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7547` |
| 4 | 4.09% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.91% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.20% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.82% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 2.67% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.19% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 10 | 1.88% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 11 | 1.41% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 13 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7436` |
| 14 | 1.25% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.19% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |

