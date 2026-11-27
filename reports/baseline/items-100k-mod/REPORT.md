# Baseline run: `items-100k-mod`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62913 -> 62913) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 15.871 / 32.716 / 106.177 / 183.881 |
| MSPT mean, ms | 22.911 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1286.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10886` |
| 2 | 5.97% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9585` |
| 4 | 2.69% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.11% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 6 | 1.96% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 7 | 1.85% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6814` |
| 8 | 1.62% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 9 | 1.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9286` |
| 10 | 1.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.34% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.12% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15340` |
| 13 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.99% | `java.util.HashMap.getNode(Object) line: 586` |
| 15 | 0.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16883` |

