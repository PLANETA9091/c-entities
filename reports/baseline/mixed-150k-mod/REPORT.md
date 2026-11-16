# Baseline run: `mixed-150k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 112322 -> 112254) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 109.483 / 195.989 / 382.322 / 550.837 |
| MSPT mean, ms | 130.781 |
| TPS estimate (1000/p50) | 9.1 |
| heap used / max, MB | 2104.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9150` |
| 2 | 8.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8307` |
| 3 | 5.72% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 5.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14070` |
| 5 | 4.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 3.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.56% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13417` |
| 8 | 1.94% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5771` |
| 10 | 1.41% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 11 | 1.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8108` |
| 13 | 1.29% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 14 | 0.94% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 0.88% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

