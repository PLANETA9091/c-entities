# Baseline run: `spread-100k-mod`

Server: `26.3-259-c7097a9 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88847 -> 88732) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 54.479 / 96.312 / 223.213 / 375.298 |
| MSPT mean, ms | 66.183 |
| TPS estimate (1000/p50) | 18.4 |
| heap used / max, MB | 1990.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10701` |
| 2 | 4.99% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15992` |
| 3 | 4.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9450` |
| 4 | 3.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.65% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 2.29% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 7 | 1.84% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15112` |
| 8 | 1.66% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6706` |
| 10 | 1.47% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 11 | 1.46% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 12 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9161` |
| 13 | 1.17% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 1.08% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15335` |
| 15 | 0.98% | `net.minecraft.world.entity.Entity.getType() line: 633` |

