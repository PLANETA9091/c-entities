# Baseline run: `mixed-150k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110307 -> 110307) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 665.984 / 810.633 / 1129.152 / 1254.221 |
| MSPT mean, ms | 692.915 |
| TPS estimate (1000/p50) | 1.5 |
| heap used / max, MB | 2109.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 36.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.28% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.92% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 4 | 2.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.87% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 8 | 1.28% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 10 | 1.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 0.99% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 0.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.90% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.79% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |

