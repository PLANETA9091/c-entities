# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62993 -> 62993) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 33.211 / 93.150 / 158.475 / 403.348 |
| MSPT mean, ms | 45.534 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1263.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7819` |
| 2 | 10.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4161` |
| 3 | 5.94% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.94% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 5 | 4.77% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7766` |
| 7 | 3.22% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2856` |
| 8 | 2.33% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2758` |
| 11 | 1.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4879` |
| 12 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.89% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 0.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4167` |
| 15 | 0.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4198` |

