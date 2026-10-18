# Baseline run: `mobs-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25055 -> 25055) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 440.557 / 483.441 / 532.153 / 564.344 |
| MSPT mean, ms | 445.505 |
| TPS estimate (1000/p50) | 2.3 |
| heap used / max, MB | 1859.1 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 25.25% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 7.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.51% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.17% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 6 | 2.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 7 | 1.61% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 10 | 1.18% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 3849` |
| 11 | 0.97% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 12 | 0.94% | `net.minecraft.world.entity.LivingEntity.doPush(Entity) line: 3738` |
| 13 | 0.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.87% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 15 | 0.83% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |

