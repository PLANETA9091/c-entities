# Baseline run: `spread-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97447 -> 97445) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 371.299 / 438.543 / 523.562 / 591.806 |
| MSPT mean, ms | 377.448 |
| TPS estimate (1000/p50) | 2.7 |
| heap used / max, MB | 2919.1 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.20% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.81% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 4 | 2.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 2.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 6 | 2.21% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 7 | 2.06% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 1.95% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 9 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 10 | 1.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.38% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.30% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 14 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.10% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |

