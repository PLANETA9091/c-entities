# Baseline run: `mobs-25k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24356 -> 22556) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 350.069 / 396.564 / 448.890 / 599.677 |
| MSPT mean, ms | 346.577 |
| TPS estimate (1000/p50) | 2.9 |
| heap used / max, MB | 1575.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 16.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 7.21% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 3 | 4.93% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 4.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 2.49% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 6 | 1.81% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.71% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 11 | 1.05% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 12 | 1.02% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 13 | 1.02% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 14 | 0.92% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 15 | 0.89% | `java.util.EnumMap.get(Object) line: 247` |

