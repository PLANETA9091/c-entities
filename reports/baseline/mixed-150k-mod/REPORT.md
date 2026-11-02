# Baseline run: `mixed-150k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109745 -> 109745) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 291.725 / 388.365 / 601.638 / 682.051 |
| MSPT mean, ms | 319.764 |
| TPS estimate (1000/p50) | 3.4 |
| heap used / max, MB | 1779.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.82% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.82% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9126` |
| 3 | 3.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6149` |
| 4 | 3.20% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.14% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.91% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9267` |
| 7 | 2.84% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 2.06% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9073` |
| 9 | 1.95% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5995` |
| 10 | 1.81% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.69% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5413` |
| 13 | 1.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

