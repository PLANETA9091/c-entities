# Baseline run: `spread-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97393 -> 97391) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 354.063 / 429.885 / 516.362 / 585.532 |
| MSPT mean, ms | 359.642 |
| TPS estimate (1000/p50) | 2.8 |
| heap used / max, MB | 3072.1 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.01% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.62% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 5 | 2.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 6 | 2.20% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 2.08% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 2.07% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 9 | 1.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 10 | 1.65% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 11 | 1.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.23% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 15 | 1.13% | `java.util.HashMap.getNode(Object) line: 585` |

