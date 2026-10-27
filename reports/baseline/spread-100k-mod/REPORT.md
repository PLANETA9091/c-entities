# Baseline run: `spread-100k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97370 -> 97370) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 177.195 / 258.893 / 355.903 / 497.514 |
| MSPT mean, ms | 189.572 |
| TPS estimate (1000/p50) | 5.6 |
| heap used / max, MB | 2047.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 2 | 3.51% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7281` |
| 3 | 3.42% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7145` |
| 5 | 3.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 6 | 3.00% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.09% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.83% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 10 | 1.68% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7092` |
| 11 | 1.37% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4060` |
| 12 | 1.01% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 13 | 1.00% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 14 | 0.97% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 15 | 0.97% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |

