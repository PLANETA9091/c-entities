# Baseline run: `spread-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97372 -> 97372) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 340.892 / 538.272 / 667.069 / 980.500 |
| MSPT mean, ms | 373.832 |
| TPS estimate (1000/p50) | 2.9 |
| heap used / max, MB | 1985.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.64% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9207` |
| 2 | 3.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 3.25% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9543` |
| 4 | 3.04% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6166` |
| 7 | 1.81% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9121` |
| 8 | 1.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5427` |
| 9 | 1.29% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.27% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 6012` |
| 11 | 1.22% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 1.06% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 15 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

