# Baseline run: `items-100k-mod`

Server: `26.3-259-c7097a9 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63048 -> 63048) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 16.386 / 29.713 / 101.401 / 259.984 |
| MSPT mean, ms | 22.773 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1198.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10701` |
| 2 | 4.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9400` |
| 4 | 3.12% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 5 | 3.08% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6256` |
| 6 | 2.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9161` |
| 8 | 2.08% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15335` |
| 9 | 1.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 11 | 1.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 13 | 0.97% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15109` |
| 14 | 0.90% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 15 | 0.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9133` |

