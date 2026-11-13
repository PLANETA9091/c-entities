# Baseline run: `mobs-25k-native`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24140 -> 24096) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 495.680 / 573.345 / 682.247 / 973.953 |
| MSPT mean, ms | 508.371 |
| TPS estimate (1000/p50) | 2.0 |
| heap used / max, MB | 1716.8 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 32.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.43% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.14% | `java.util.HashMap.getNode(Object) line: 586` |
| 5 | 1.95% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.37% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 8 | 0.94% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.81% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 0.77% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 12 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 0.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 0.66% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 15 | 0.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

