# Baseline run: `spread-100k-mod`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88845 -> 88722) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 47.311 / 69.924 / 200.580 / 345.246 |
| MSPT mean, ms | 57.191 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 2066.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10829` |
| 2 | 4.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9578` |
| 3 | 3.94% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.23% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 5 | 2.61% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 2.54% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 7 | 1.85% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15240` |
| 8 | 1.78% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6770` |
| 9 | 1.68% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16449` |
| 10 | 1.45% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 11 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9274` |
| 12 | 1.27% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 13 | 1.21% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.07% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6029` |
| 15 | 1.05% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |

