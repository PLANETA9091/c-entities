# Baseline run: `items-100k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62980 -> 62981) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 24.091 / 45.344 / 126.812 / 298.443 |
| MSPT mean, ms | 33.212 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1336.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9942` |
| 2 | 5.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 5.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8799` |
| 4 | 4.65% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.00% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1018` |
| 6 | 1.66% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1067` |
| 7 | 1.63% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1019` |
| 8 | 1.51% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14481` |
| 9 | 1.49% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14279` |
| 10 | 1.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6116` |
| 11 | 1.25% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.13% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8585` |
| 14 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

