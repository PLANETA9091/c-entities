# Baseline run: `spread-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97481 -> 97481) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 261.290 / 402.586 / 516.498 / 738.817 |
| MSPT mean, ms | 289.401 |
| TPS estimate (1000/p50) | 3.8 |
| heap used / max, MB | 2850.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.04% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.87% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8929` |
| 3 | 3.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.83% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 7 | 2.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 8 | 2.05% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.94% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 10 | 1.73% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5679` |
| 11 | 1.64% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.19% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 1.18% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

