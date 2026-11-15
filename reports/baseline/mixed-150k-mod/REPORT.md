# Baseline run: `mixed-150k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 112412 -> 112376) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 84.208 / 133.966 / 286.836 / 392.010 |
| MSPT mean, ms | 97.671 |
| TPS estimate (1000/p50) | 11.9 |
| heap used / max, MB | 2089.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.87% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 7.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7913` |
| 3 | 6.77% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 5.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8724` |
| 5 | 5.88% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13644` |
| 6 | 3.32% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 3.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.86% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5384` |
| 11 | 1.48% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5388` |
| 12 | 1.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7717` |
| 13 | 1.36% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 1.20% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 15 | 1.06% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |

