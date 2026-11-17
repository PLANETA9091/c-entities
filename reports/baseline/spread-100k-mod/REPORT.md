# Baseline run: `spread-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88750 -> 88616) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 54.578 / 109.373 / 216.742 / 278.843 |
| MSPT mean, ms | 65.969 |
| TPS estimate (1000/p50) | 18.3 |
| heap used / max, MB | 2448.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9568` |
| 2 | 8.26% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.09% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14614` |
| 4 | 5.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8775` |
| 5 | 2.88% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 1.58% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 7 | 1.51% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 984` |
| 8 | 1.50% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13849` |
| 9 | 1.50% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 1.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6038` |
| 11 | 1.27% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 12 | 1.16% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.10% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 0.94% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6042` |
| 15 | 0.91% | `net.minecraft.world.entity.Entity.getType() line: 633` |

