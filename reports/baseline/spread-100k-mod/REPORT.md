# Baseline run: `spread-100k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88763 -> 88638) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 54.239 / 93.797 / 191.777 / 264.382 |
| MSPT mean, ms | 64.191 |
| TPS estimate (1000/p50) | 18.4 |
| heap used / max, MB | 1955.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10077` |
| 2 | 4.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8984` |
| 3 | 4.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15283` |
| 5 | 2.45% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 2.40% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 7 | 1.87% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14417` |
| 8 | 1.55% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 9 | 1.42% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 1.37% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1018` |
| 11 | 1.33% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6485` |
| 12 | 1.32% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 13 | 1.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8673` |
| 14 | 1.09% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 0.97% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

