# Baseline run: `items-100k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62941 -> 62941) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 13.125 / 22.276 / 83.813 / 192.788 |
| MSPT mean, ms | 16.948 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1354.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.21% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1016` |
| 2 | 4.82% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 4.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10077` |
| 5 | 3.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8934` |
| 6 | 2.66% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14628` |
| 7 | 1.95% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1018` |
| 8 | 1.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.09% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1017` |
| 11 | 1.01% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14414` |
| 12 | 0.92% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8700` |
| 13 | 0.92% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6485` |
| 14 | 0.89% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 15 | 0.86% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |

