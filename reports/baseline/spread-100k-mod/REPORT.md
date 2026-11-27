# Baseline run: `spread-100k-mod`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88724 -> 88607) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 46.968 / 71.550 / 197.154 / 348.417 |
| MSPT mean, ms | 56.062 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 2295.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10886` |
| 2 | 4.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9635` |
| 3 | 3.90% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.15% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 5 | 2.31% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 6 | 2.13% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 7 | 2.09% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15343` |
| 8 | 1.62% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 9 | 1.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16623` |
| 10 | 1.50% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6814` |
| 11 | 1.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9286` |
| 12 | 1.38% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.05% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13967` |
| 14 | 0.95% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 0.89% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

