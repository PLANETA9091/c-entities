# Baseline run: `spread-100k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88790 -> 88673) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 49.379 / 81.758 / 204.345 / 299.909 |
| MSPT mean, ms | 59.966 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1949.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10309` |
| 2 | 5.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9132` |
| 3 | 4.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15535` |
| 4 | 3.85% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.77% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 2.65% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 7 | 1.91% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14655` |
| 8 | 1.73% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6600` |
| 9 | 1.67% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 1.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8843` |
| 11 | 1.48% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 12 | 1.44% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.32% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 14 | 1.04% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 1.02% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1024` |

