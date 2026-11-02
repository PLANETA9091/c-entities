# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62975 -> 62975) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 34.547 / 91.968 / 170.869 / 273.766 |
| MSPT mean, ms | 45.973 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1185.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9207` |
| 2 | 8.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5427` |
| 3 | 5.57% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 4 | 4.80% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 4.25% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 4.08% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9121` |
| 7 | 3.76% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4020` |
| 8 | 2.66% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 1.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3893` |
| 10 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5433` |
| 12 | 0.97% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 0.92% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.84% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3973` |

