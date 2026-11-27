# Baseline run: `mixed-150k-mod`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111653 -> 111200) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 50.551 / 90.341 / 193.698 / 273.738 |
| MSPT mean, ms | 61.046 |
| TPS estimate (1000/p50) | 19.8 |
| heap used / max, MB | 1840.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 6.42% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16623` |
| 3 | 6.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10886` |
| 4 | 3.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.90% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 6 | 2.68% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16642` |
| 7 | 2.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9585` |
| 8 | 2.15% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 9 | 1.72% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15343` |
| 10 | 1.36% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 11 | 1.23% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 12 | 1.21% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6077` |
| 14 | 1.00% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3284` |
| 15 | 0.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9286` |

