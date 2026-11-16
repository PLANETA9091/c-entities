# Baseline run: `mixed-150k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 112261 -> 112157) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 81.994 / 127.920 / 257.158 / 370.779 |
| MSPT mean, ms | 96.338 |
| TPS estimate (1000/p50) | 12.2 |
| heap used / max, MB | 2342.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9150` |
| 2 | 8.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8307` |
| 3 | 6.02% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 5.97% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14070` |
| 5 | 5.03% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 4.76% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.78% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5771` |
| 9 | 1.57% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13417` |
| 10 | 1.45% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 11 | 1.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.30% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.18% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 1.00% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2840` |
| 15 | 1.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5775` |

