# Baseline run: `spread-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 96135 -> 96000) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 540.339 / 648.964 / 757.775 / 904.459 |
| MSPT mean, ms | 551.975 |
| TPS estimate (1000/p50) | 1.9 |
| heap used / max, MB | 2236.0 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 2.94% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 2 | 2.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 2.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.04% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 5 | 1.91% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 6 | 1.90% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 7 | 1.62% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.43% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 9 | 1.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.24% | `java.lang.Enum.ordinal() line: 128` |
| 13 | 1.19% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 14 | 1.09% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 15 | 1.07% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |

