# Baseline run: `items-100k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63049 -> 63048) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 16.990 / 35.069 / 110.351 / 226.717 |
| MSPT mean, ms | 23.997 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1406.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10555` |
| 2 | 5.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 5.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9255` |
| 4 | 3.12% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 5 | 2.36% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.01% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6632` |
| 7 | 1.92% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 8 | 1.82% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15189` |
| 9 | 1.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9016` |
| 10 | 1.69% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 11 | 1.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.11% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.09% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14963` |
| 14 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.77% | `net.minecraft.world.entity.Entity.getX() line: 5100` |

