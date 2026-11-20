# Baseline run: `mixed-150k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111670 -> 111241) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 61.345 / 106.528 / 234.534 / 286.832 |
| MSPT mean, ms | 73.272 |
| TPS estimate (1000/p50) | 16.3 |
| heap used / max, MB | 1752.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10137` |
| 2 | 7.60% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.57% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15343` |
| 4 | 3.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8994` |
| 5 | 3.30% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.23% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14477` |
| 7 | 1.97% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15364` |
| 8 | 1.94% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1018` |
| 9 | 1.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8755` |
| 10 | 1.83% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 11 | 1.52% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 12 | 1.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.43% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 14 | 1.40% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 1.09% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5896` |

