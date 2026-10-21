# Baseline run: `mobs-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25049 -> 25045) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 567.236 / 619.174 / 681.277 / 733.572 |
| MSPT mean, ms | 571.004 |
| TPS estimate (1000/p50) | 1.8 |
| heap used / max, MB | 1752.0 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 21.00% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 3 | 3.80% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 6 | 1.80% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 7 | 1.77% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 9 | 0.93% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 0.88% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 11 | 0.88% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 12 | 0.87% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 13 | 0.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.69% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |

