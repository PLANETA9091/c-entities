# Baseline run: `items-100k-mod`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63022 -> 63022) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 14.223 / 25.396 / 99.720 / 2285.328 |
| MSPT mean, ms | 22.023 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1571.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9528` |
| 2 | 6.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10829` |
| 3 | 5.42% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.77% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.54% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 6 | 2.54% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 7 | 1.83% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6770` |
| 8 | 1.34% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15237` |
| 9 | 1.18% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 10 | 1.13% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 11 | 1.07% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16709` |
| 13 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.86% | `net.minecraft.world.entity.Entity.getX() line: 5100` |

