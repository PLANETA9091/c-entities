# Baseline run: `mobs-25k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25058 -> 25058) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 206.093 / 299.484 / 396.148 / 477.747 |
| MSPT mean, ms | 219.088 |
| TPS estimate (1000/p50) | 4.9 |
| heap used / max, MB | 1190.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.85% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.16% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.83% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 1.72% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6965` |
| 5 | 1.59% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6916` |
| 6 | 1.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 9 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.09% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.00% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 0.99% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 0.96% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4381` |
| 14 | 0.96% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 15 | 0.96% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |

