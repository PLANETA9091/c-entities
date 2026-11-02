# Baseline run: `spread-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97367 -> 97367) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 295.773 / 453.509 / 547.410 / 743.248 |
| MSPT mean, ms | 321.188 |
| TPS estimate (1000/p50) | 3.4 |
| heap used / max, MB | 2365.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.63% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9877` |
| 2 | 3.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 3.04% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9374` |
| 4 | 2.96% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6230` |
| 6 | 2.34% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5461` |
| 8 | 1.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.41% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 10 | 1.41% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.41% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.17% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 1.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.13% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |

