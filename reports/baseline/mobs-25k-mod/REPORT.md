# Baseline run: `mobs-25k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25050 -> 25050) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 197.702 / 347.203 / 548.447 / 663.128 |
| MSPT mean, ms | 226.199 |
| TPS estimate (1000/p50) | 5.1 |
| heap used / max, MB | 1551.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.38% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.01% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 1.31% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 6 | 1.27% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 9 | 1.07% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 10 | 1.03% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 0.95% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 0.94% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 15 | 0.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |

