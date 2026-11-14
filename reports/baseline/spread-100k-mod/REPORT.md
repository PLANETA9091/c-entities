# Baseline run: `spread-100k-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88761 -> 88625) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 89.980 / 138.245 / 271.531 / 379.883 |
| MSPT mean, ms | 102.444 |
| TPS estimate (1000/p50) | 11.1 |
| heap used / max, MB | 2213.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8219` |
| 2 | 3.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7466` |
| 3 | 3.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13025` |
| 4 | 3.36% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 2.88% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 2.28% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.01% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.67% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 1.67% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.48% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 11 | 1.38% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4986` |
| 12 | 1.33% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 13 | 1.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5191` |
| 14 | 1.04% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 1.04% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |

