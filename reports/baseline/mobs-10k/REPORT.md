# Baseline run: `mobs-10k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 10000 (items 0 / stands 0 / zombies 10000) (merge shrink: 10025 -> 10019) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 131.028 / 144.798 / 181.457 / 202.373 |
| MSPT mean, ms | 132.890 |
| TPS estimate (1000/p50) | 7.6 |
| heap used / max, MB | 1508.6 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.80% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.09% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 1.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 5 | 1.27% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.24% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 8 | 1.09% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 9 | 1.09% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 10 | 0.95% | `io.netty.util.internal.shaded.org.jctools.util.UnsafeRefArrayAccess.soRefElement(Object[], long, Object) line: 63` |
| 11 | 0.88% | `java.util.EnumMap.get(Object) line: 247` |
| 12 | 0.87% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 13 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processPendingFullUpdate() line: 1501` |
| 14 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.78% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |

