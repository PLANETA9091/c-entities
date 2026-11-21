# Baseline run: `spread-100k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88835 -> 88698) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 40.174 / 61.249 / 158.169 / 202.748 |
| MSPT mean, ms | 47.390 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 2012.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10231` |
| 2 | 6.07% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9073` |
| 4 | 4.90% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15438` |
| 5 | 2.28% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14572` |
| 6 | 2.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8784` |
| 7 | 2.08% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 8 | 1.63% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 1.45% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6541` |
| 10 | 1.45% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.43% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 12 | 1.39% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 13 | 1.31% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 1.25% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5904` |
| 15 | 1.25% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1024` |

