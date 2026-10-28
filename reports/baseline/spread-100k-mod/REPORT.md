# Baseline run: `spread-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97404 -> 97404) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 198.926 / 348.248 / 544.304 / 827.382 |
| MSPT mean, ms | 228.682 |
| TPS estimate (1000/p50) | 5.0 |
| heap used / max, MB | 2093.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.62% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7863` |
| 2 | 3.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 3 | 3.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 3.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 7 | 2.09% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.52% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 10 | 1.40% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4637` |
| 11 | 1.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4073` |
| 12 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 0.88% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 14 | 0.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 0.87% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

