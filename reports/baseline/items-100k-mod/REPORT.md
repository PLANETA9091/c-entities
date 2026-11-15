# Baseline run: `items-100k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63045 -> 63045) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 30.012 / 51.268 / 120.504 / 235.527 |
| MSPT mean, ms | 37.916 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1684.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 16.05% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7819` |
| 2 | 4.91% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5291` |
| 4 | 4.50% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8581` |
| 5 | 4.00% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.72% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12838` |
| 7 | 3.01% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 2.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13598` |
| 9 | 1.97% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 1.95% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13312` |
| 11 | 1.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5295` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5297` |
| 13 | 1.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7613` |
| 14 | 1.13% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

