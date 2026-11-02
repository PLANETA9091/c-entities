# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62906 -> 62906) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 36.453 / 90.894 / 162.215 / 186.735 |
| MSPT mean, ms | 47.908 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1441.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 15.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9207` |
| 2 | 9.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5427` |
| 3 | 7.15% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.80% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 5 | 3.99% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4020` |
| 6 | 3.67% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.32% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9121` |
| 8 | 1.18% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.12% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 11 | 1.07% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9166` |
| 12 | 1.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6166` |
| 13 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3893` |

