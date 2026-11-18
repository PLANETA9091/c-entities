# Baseline run: `items-100k-mod`

Server: `26.3-255-c22378f (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63003 -> 63003) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 22.367 / 47.887 / 141.378 / 225.653 |
| MSPT mean, ms | 32.335 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1841.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9870` |
| 2 | 6.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8727` |
| 3 | 5.64% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 5.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 4.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 6 | 3.50% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 984` |
| 7 | 2.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8513` |
| 8 | 1.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6044` |
| 9 | 1.61% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14400` |
| 10 | 1.39% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.19% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14207` |
| 12 | 1.17% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3009` |
| 13 | 1.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5998` |
| 14 | 0.99% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 0.88% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14975` |

