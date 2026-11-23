# Baseline run: `spread-100k-mod`

Server: `26.3-259-c7097a9 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88819 -> 88704) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 33.436 / 55.712 / 151.095 / 187.401 |
| MSPT mean, ms | 41.090 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1952.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.88% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16008` |
| 3 | 4.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10717` |
| 4 | 3.83% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 5 | 3.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9466` |
| 6 | 2.31% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6253` |
| 7 | 1.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9162` |
| 8 | 1.86% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16029` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 10 | 1.19% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15128` |
| 11 | 1.11% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 12 | 1.09% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 0.92% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 14 | 0.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.89% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |

