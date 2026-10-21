# Baseline run: `items-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63147 -> 63138) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 100.525 / 179.387 / 217.740 / 302.914 |
| MSPT mean, ms | 128.732 |
| TPS estimate (1000/p50) | 9.9 |
| heap used / max, MB | 1895.1 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 18.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 4.01% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 3.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 3.50% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 6 | 3.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 7 | 2.48% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 2.05% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 1.87% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 10 | 1.84% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 11 | 1.81% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 13 | 1.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 1.07% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 15 | 1.03% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |

