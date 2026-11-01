# Baseline run: `spread-100k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97408 -> 97408) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 401.900 / 464.148 / 679.901 / 812.300 |
| MSPT mean, ms | 415.148 |
| TPS estimate (1000/p50) | 2.5 |
| heap used / max, MB | 2154.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.41% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.54% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.26% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.14% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 2.13% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.00% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.73% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 10 | 1.69% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.62% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.56% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.33% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 1.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 1.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

