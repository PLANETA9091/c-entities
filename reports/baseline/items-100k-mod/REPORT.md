# Baseline run: `items-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62969 -> 62969) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 23.709 / 46.888 / 120.957 / 277.528 |
| MSPT mean, ms | 31.337 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1698.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9568` |
| 2 | 4.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 4.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8725` |
| 4 | 4.40% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 984` |
| 5 | 4.20% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.63% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1033` |
| 7 | 1.59% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14039` |
| 8 | 1.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 9 | 1.50% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 985` |
| 10 | 1.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6042` |
| 11 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8511` |
| 12 | 1.10% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13846` |
| 13 | 0.88% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14614` |
| 14 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.81% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |

