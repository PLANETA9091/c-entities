# Baseline run: `spread-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97403 -> 97403) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 275.885 / 438.221 / 557.542 / 767.507 |
| MSPT mean, ms | 306.338 |
| TPS estimate (1000/p50) | 3.6 |
| heap used / max, MB | 2610.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.95% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.68% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9334` |
| 3 | 3.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9193` |
| 4 | 3.19% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6152` |
| 7 | 2.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.05% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9107` |
| 9 | 1.73% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5998` |
| 10 | 1.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.42% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5416` |
| 14 | 1.30% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 15 | 1.21% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

