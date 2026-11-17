# Baseline run: `mixed-150k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111661 -> 111315) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 62.028 / 100.453 / 230.022 / 323.308 |
| MSPT mean, ms | 75.724 |
| TPS estimate (1000/p50) | 16.1 |
| heap used / max, MB | 2403.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9373` |
| 2 | 9.53% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 3 | 4.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14355` |
| 4 | 4.78% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8530` |
| 6 | 3.40% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.06% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13654` |
| 9 | 2.03% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7567` |
| 10 | 1.62% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 11 | 1.43% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14392` |
| 13 | 1.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 14 | 1.23% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 1.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8316` |

