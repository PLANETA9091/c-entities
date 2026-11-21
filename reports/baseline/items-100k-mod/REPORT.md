# Baseline run: `items-100k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62966 -> 62965) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 16.590 / 30.370 / 105.424 / 178.989 |
| MSPT mean, ms | 23.324 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1261.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10231` |
| 2 | 7.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9023` |
| 3 | 5.84% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 4.12% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.05% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1024` |
| 6 | 2.65% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 7 | 1.67% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14783` |
| 8 | 1.48% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14569` |
| 9 | 1.41% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6541` |
| 10 | 1.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 11 | 1.36% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 12 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.98% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.95% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 15 | 0.95% | `net.minecraft.world.entity.Entity.getX() line: 5100` |

