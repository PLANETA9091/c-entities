# Baseline run: `spread-100k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88711 -> 88584) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 54.560 / 97.869 / 214.840 / 282.533 |
| MSPT mean, ms | 65.926 |
| TPS estimate (1000/p50) | 18.3 |
| heap used / max, MB | 2155.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10137` |
| 2 | 4.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9044` |
| 3 | 4.53% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.51% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15343` |
| 5 | 2.57% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 1.89% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14477` |
| 7 | 1.81% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 8 | 1.70% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1018` |
| 9 | 1.68% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 10 | 1.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8755` |
| 11 | 1.42% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.25% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 13 | 1.09% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 1.04% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 1.03% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |

