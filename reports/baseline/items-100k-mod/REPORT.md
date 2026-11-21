# Baseline run: `items-100k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62856 -> 62857) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 19.390 / 39.421 / 114.028 / 275.173 |
| MSPT mean, ms | 26.914 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1161.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10309` |
| 2 | 6.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9082` |
| 3 | 5.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.50% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.48% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1024` |
| 6 | 2.16% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6600` |
| 7 | 2.10% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 8 | 1.96% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14878` |
| 9 | 1.80% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 10 | 1.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8843` |
| 11 | 1.24% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 12 | 1.12% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1022` |
| 13 | 1.04% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14652` |
| 14 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 1.02% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |

