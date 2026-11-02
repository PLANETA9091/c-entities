# Baseline run: `mobs-25k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25051 -> 25051) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 284.609 / 481.477 / 661.080 / 814.173 |
| MSPT mean, ms | 329.030 |
| TPS estimate (1000/p50) | 3.5 |
| heap used / max, MB | 1449.8 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 2.36% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10182` |
| 2 | 1.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 1.58% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 1.34% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9580` |
| 5 | 1.34% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.32% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.31% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 8 | 1.27% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.25% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.19% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 0.98% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6812` |
| 13 | 0.97% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6880` |
| 14 | 0.93% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 15 | 0.85% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |

