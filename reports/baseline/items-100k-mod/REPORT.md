# Baseline run: `items-100k-mod`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62928 -> 62928) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 12.057 / 24.181 / 91.730 / 148.422 |
| MSPT mean, ms | 17.231 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1266.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.39% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9530` |
| 3 | 4.39% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 4 | 4.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10831` |
| 5 | 3.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9276` |
| 6 | 3.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.96% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 8 | 1.81% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6772` |
| 9 | 1.30% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 10 | 1.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.04% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.04% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.04% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 14 | 1.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9248` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15285` |

