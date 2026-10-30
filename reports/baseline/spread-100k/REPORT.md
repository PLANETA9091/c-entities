# Baseline run: `spread-100k`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97420 -> 97420) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 454.548 / 539.322 / 764.083 / 900.293 |
| MSPT mean, ms | 469.581 |
| TPS estimate (1000/p50) | 2.2 |
| heap used / max, MB | 2090.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 16.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.53% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.00% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 5 | 2.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.59% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 8 | 1.57% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 9 | 1.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.43% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.39% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 1.28% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 15 | 1.27% | `java.util.concurrent.CompletableFuture.isDone() line: 2076` |

