# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63050 -> 63050) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 31.805 / 76.410 / 136.598 / 172.721 |
| MSPT mean, ms | 41.052 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1347.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.83% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8933` |
| 2 | 6.89% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 3 | 5.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8986` |
| 4 | 4.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5273` |
| 5 | 3.59% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6019` |
| 7 | 2.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3739` |
| 8 | 2.04% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3866` |
| 9 | 1.76% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 10 | 1.55% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3819` |
| 11 | 1.53% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 12 | 1.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5279` |
| 14 | 1.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |

