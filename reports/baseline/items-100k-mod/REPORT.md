# Baseline run: `items-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62968 -> 62969) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 33.367 / 58.309 / 129.071 / 338.849 |
| MSPT mean, ms | 42.170 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1665.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7913` |
| 2 | 8.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8724` |
| 3 | 5.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 4.14% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.82% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7733` |
| 7 | 3.08% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 8 | 2.14% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13496` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5384` |
| 10 | 1.20% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12988` |
| 11 | 1.13% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13918` |
| 13 | 0.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.76% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13644` |

