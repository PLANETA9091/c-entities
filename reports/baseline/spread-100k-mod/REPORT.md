# Baseline run: `spread-100k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88803 -> 88648) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 49.399 / 89.632 / 198.602 / 345.540 |
| MSPT mean, ms | 59.302 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1965.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10331` |
| 2 | 5.09% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15622` |
| 3 | 4.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9132` |
| 4 | 4.22% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.63% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 2.17% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 7 | 1.94% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14742` |
| 8 | 1.72% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 9 | 1.49% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6600` |
| 10 | 1.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8843` |
| 11 | 1.30% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 12 | 1.21% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.06% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 14 | 1.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 0.91% | `net.minecraft.world.entity.Entity.getType() line: 633` |

