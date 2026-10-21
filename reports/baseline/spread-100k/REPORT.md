# Baseline run: `spread-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97338 -> 97332) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 553.194 / 642.874 / 782.647 / 835.001 |
| MSPT mean, ms | 560.566 |
| TPS estimate (1000/p50) | 1.8 |
| heap used / max, MB | 2306.6 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 4 | 2.01% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.69% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 1.41% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 1.36% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 9 | 1.20% | `java.util.HashMap.getNode(Object) line: 585` |
| 10 | 1.19% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 11 | 1.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.15% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 13 | 0.99% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 14 | 0.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.95% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |

