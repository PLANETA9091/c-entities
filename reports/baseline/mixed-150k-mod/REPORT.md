# Baseline run: `mixed-150k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111893 -> 111397) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 67.412 / 113.776 / 286.798 / 394.271 |
| MSPT mean, ms | 83.357 |
| TPS estimate (1000/p50) | 14.8 |
| heap used / max, MB | 1734.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10077` |
| 2 | 6.77% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15283` |
| 3 | 5.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8934` |
| 5 | 3.26% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.41% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14417` |
| 7 | 1.89% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1018` |
| 8 | 1.68% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 9 | 1.51% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 10 | 1.17% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6485` |
| 11 | 1.15% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 1.13% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.12% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 14 | 1.04% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8673` |
| 15 | 1.00% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5867` |

