# Baseline run: `spread-100k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97443 -> 97443) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 208.146 / 359.326 / 589.204 / 729.283 |
| MSPT mean, ms | 242.229 |
| TPS estimate (1000/p50) | 4.8 |
| heap used / max, MB | 2172.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.67% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8183` |
| 2 | 3.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 3.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8042` |
| 4 | 3.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5078` |
| 5 | 2.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.09% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 2.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.93% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7989` |
| 10 | 1.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4360` |
| 11 | 1.42% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.32% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4933` |
| 14 | 1.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.18% | `java.util.ArrayDeque.pollFirst() line: 378` |

