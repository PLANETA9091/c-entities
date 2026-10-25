# Baseline run: `mobs-25k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25043 -> 25043) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 149.904 / 292.603 / 372.073 / 407.335 |
| MSPT mean, ms | 168.910 |
| TPS estimate (1000/p50) | 6.7 |
| heap used / max, MB | 1427.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.96% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.08% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1484` |
| 3 | 1.72% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 3872` |
| 4 | 1.48% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 1.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 6 | 1.32% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3517` |
| 7 | 1.30% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.19% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.12% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 0.96% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 14 | 0.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 0.81% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |

