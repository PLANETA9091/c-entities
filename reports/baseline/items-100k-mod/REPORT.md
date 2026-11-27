# Baseline run: `items-100k-mod`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62929 -> 62929) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 16.121 / 34.745 / 117.160 / 220.072 |
| MSPT mean, ms | 23.822 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1346.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10834` |
| 2 | 6.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9533` |
| 3 | 5.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.65% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 5 | 2.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.36% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 7 | 1.80% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6775` |
| 8 | 1.34% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 9 | 1.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.05% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15288` |
| 12 | 0.91% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 0.89% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 14 | 0.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9251` |
| 15 | 0.87% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |

