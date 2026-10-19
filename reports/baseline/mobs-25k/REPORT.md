# Baseline run: `mobs-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25060 -> 25060) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 488.016 / 517.949 / 561.582 / 663.916 |
| MSPT mean, ms | 490.724 |
| TPS estimate (1000/p50) | 2.0 |
| heap used / max, MB | 2071.7 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 26.34% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.23% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.85% | `java.util.HashMap.getNode(Object) line: 585` |
| 5 | 2.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 1.35% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 7 | 1.30% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.04% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 10 | 1.03% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 11 | 1.00% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 12 | 0.89% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 13 | 0.85% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 3849` |
| 14 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.78% | `java.util.EnumMap.get(Object) line: 247` |

