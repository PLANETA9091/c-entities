# Baseline run: `spread-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 93138 -> 93010) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 571.381 / 700.015 / 825.431 / 972.457 |
| MSPT mean, ms | 581.591 |
| TPS estimate (1000/p50) | 1.8 |
| heap used / max, MB | 2106.5 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.32% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 4 | 1.99% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.53% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 7 | 1.41% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 1.32% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 9 | 1.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 1.16% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 11 | 1.10% | `java.util.HashMap.getNode(Object) line: 585` |
| 12 | 1.10% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.07% | `java.lang.Enum.ordinal() line: 128` |
| 14 | 1.04% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

