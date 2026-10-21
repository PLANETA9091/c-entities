# Baseline run: `mobs-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25070 -> 25067) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 457.213 / 504.181 / 536.698 / 596.841 |
| MSPT mean, ms | 462.957 |
| TPS estimate (1000/p50) | 2.2 |
| heap used / max, MB | 1375.1 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 20.41% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 4.60% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.51% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 1.71% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 1.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 8 | 1.17% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 9 | 0.99% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 0.94% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 11 | 0.93% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 12 | 0.89% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 13 | 0.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.79% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 15 | 0.76% | `java.util.EnumMap.get(Object) line: 247` |

