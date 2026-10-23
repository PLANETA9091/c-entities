# Baseline run: `spread-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97259 -> 97157) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 380.754 / 452.990 / 519.056 / 703.253 |
| MSPT mean, ms | 391.945 |
| TPS estimate (1000/p50) | 2.6 |
| heap used / max, MB | 2475.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.69% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 5 | 2.66% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 2.49% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.27% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 8 | 2.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 9 | 1.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 1.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.45% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 12 | 1.27% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 13 | 1.21% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 14 | 1.21% | `java.util.HashMap.getNode(Object) line: 585` |
| 15 | 1.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |

