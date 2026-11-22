# Baseline run: `spread-100k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88787 -> 88657) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 42.144 / 64.396 / 190.250 / 253.082 |
| MSPT mean, ms | 50.755 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 2111.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10555` |
| 2 | 5.65% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15846` |
| 3 | 5.23% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.90% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9305` |
| 5 | 3.46% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 2.12% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14966` |
| 7 | 1.87% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 8 | 1.67% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 9 | 1.62% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15867` |
| 10 | 1.62% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 11 | 1.33% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 12 | 1.10% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6632` |
| 13 | 1.05% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 1.00% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 15 | 0.96% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5942` |

