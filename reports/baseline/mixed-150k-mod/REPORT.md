# Baseline run: `mixed-150k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 112362 -> 112268) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 77.110 / 113.658 / 235.440 / 284.984 |
| MSPT mean, ms | 89.122 |
| TPS estimate (1000/p50) | 13.0 |
| heap used / max, MB | 2156.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8794` |
| 2 | 8.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 3 | 6.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13714` |
| 4 | 5.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7983` |
| 5 | 4.07% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 3.96% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.90% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5454` |
| 9 | 1.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7791` |
| 10 | 1.79% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13061` |
| 11 | 1.56% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.27% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.20% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 1.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5458` |
| 15 | 1.17% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |

