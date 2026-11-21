# Baseline run: `spread-100k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88797 -> 88663) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 49.272 / 99.456 / 213.182 / 299.241 |
| MSPT mean, ms | 60.920 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 2103.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10331` |
| 2 | 5.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9132` |
| 3 | 4.88% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15622` |
| 4 | 3.94% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.16% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 2.04% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 7 | 1.90% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14742` |
| 8 | 1.50% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 9 | 1.50% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6600` |
| 10 | 1.35% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 11 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8843` |
| 12 | 1.22% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.20% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 14 | 1.02% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 0.87% | `net.minecraft.world.entity.Entity.getType() line: 633` |

