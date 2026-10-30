# Baseline run: `mixed-150k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109800 -> 109800) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 295.769 / 417.985 / 527.270 / 681.584 |
| MSPT mean, ms | 324.166 |
| TPS estimate (1000/p50) | 3.4 |
| heap used / max, MB | 1703.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.29% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8282` |
| 3 | 3.34% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8423` |
| 4 | 2.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5318` |
| 6 | 2.79% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.42% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8229` |
| 8 | 1.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4594` |
| 9 | 1.70% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.44% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.42% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5173` |
| 14 | 1.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

