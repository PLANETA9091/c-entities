# Baseline run: `spread-100k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88842 -> 88706) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 38.618 / 58.139 / 166.754 / 227.422 |
| MSPT mean, ms | 46.315 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 2150.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.65% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15127` |
| 3 | 4.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9942` |
| 4 | 3.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8849` |
| 5 | 3.12% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 1.97% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1016` |
| 7 | 1.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15145` |
| 8 | 1.45% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14282` |
| 9 | 1.40% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6116` |
| 11 | 1.11% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1064` |
| 12 | 1.01% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 13 | 0.99% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 0.99% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 15 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

