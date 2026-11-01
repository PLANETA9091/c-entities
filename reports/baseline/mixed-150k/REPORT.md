# Baseline run: `mixed-150k`

Server: `26.3-236-f119026 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110371 -> 110371) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 487.349 / 597.915 / 857.736 / 1015.170 |
| MSPT mean, ms | 508.966 |
| TPS estimate (1000/p50) | 2.1 |
| heap used / max, MB | 1477.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 27.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.70% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.55% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.13% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.04% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.73% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 2.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 8 | 1.68% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 10 | 1.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.22% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 12 | 1.21% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.17% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 14 | 1.17% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.14% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |

