# Baseline run: `mixed-150k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 112312 -> 112226) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 85.417 / 138.178 / 274.232 / 417.381 |
| MSPT mean, ms | 98.702 |
| TPS estimate (1000/p50) | 11.7 |
| heap used / max, MB | 2332.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 8.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8898` |
| 3 | 6.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8055` |
| 4 | 5.85% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13818` |
| 5 | 4.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.17% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.51% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5519` |
| 9 | 1.76% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.73% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13165` |
| 11 | 1.63% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7834` |
| 13 | 1.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5523` |
| 14 | 1.36% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 1.33% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |

