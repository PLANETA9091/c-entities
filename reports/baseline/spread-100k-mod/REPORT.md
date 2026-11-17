# Baseline run: `spread-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88718 -> 88597) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 57.091 / 99.379 / 227.142 / 287.727 |
| MSPT mean, ms | 68.998 |
| TPS estimate (1000/p50) | 17.5 |
| heap used / max, MB | 2480.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9503` |
| 2 | 6.07% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.91% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14509` |
| 4 | 4.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8710` |
| 5 | 2.44% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 6 | 2.20% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 7 | 1.94% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7697` |
| 8 | 1.69% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13784` |
| 9 | 1.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5990` |
| 10 | 1.46% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 11 | 1.20% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 1.17% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 0.99% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 14 | 0.94% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 0.88% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

