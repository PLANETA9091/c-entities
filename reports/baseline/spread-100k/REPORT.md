# Baseline run: `spread-100k`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97436 -> 97435) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 340.908 / 428.363 / 587.105 / 664.935 |
| MSPT mean, ms | 352.663 |
| TPS estimate (1000/p50) | 2.9 |
| heap used / max, MB | 2591.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.70% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.04% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 1.96% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 1.82% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 8 | 1.75% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 9 | 1.64% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 11 | 1.63% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 1.27% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.18% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

