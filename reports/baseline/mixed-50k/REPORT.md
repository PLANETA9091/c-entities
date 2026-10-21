# Baseline run: `mixed-50k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 50000 (items 34897 / stands 10136 / zombies 4967) (merge shrink: 41581 -> 41578) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 142.555 / 161.899 / 195.138 / 245.405 |
| MSPT mean, ms | 146.174 |
| TPS estimate (1000/p50) | 7.0 |
| heap used / max, MB | 1897.8 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 24.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 1.86% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 1.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 5 | 1.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 7 | 1.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.02% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 9 | 0.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 0.89% | `java.lang.ThreadLocal.get() line: 172` |
| 11 | 0.77% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 12 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 13 | 0.74% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 14 | 0.72% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 15 | 0.71% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |

