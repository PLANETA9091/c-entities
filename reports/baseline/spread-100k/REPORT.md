# Baseline run: `spread-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97452 -> 97446) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 475.375 / 556.604 / 647.309 / 745.170 |
| MSPT mean, ms | 479.917 |
| TPS estimate (1000/p50) | 2.1 |
| heap used / max, MB | 2204.5 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.18% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 5 | 1.87% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.42% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 1.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.replaceFrom(AttributeInstance) line: 181` |
| 11 | 1.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 13 | 1.07% | `java.util.HashMap.getNode(Object) line: 585` |
| 14 | 1.05% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 15 | 0.96% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |

