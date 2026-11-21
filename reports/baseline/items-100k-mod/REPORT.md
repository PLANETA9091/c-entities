# Baseline run: `items-100k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63045 -> 63045) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 17.259 / 27.202 / 101.424 / 242.537 |
| MSPT mean, ms | 22.730 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1332.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.74% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10331` |
| 3 | 4.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9082` |
| 4 | 3.36% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6200` |
| 5 | 3.23% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1024` |
| 6 | 2.91% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.23% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14965` |
| 8 | 2.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8843` |
| 9 | 1.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.49% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 11 | 1.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.22% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1025` |
| 13 | 1.11% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14739` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1043` |
| 15 | 0.97% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6600` |

