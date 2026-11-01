# Baseline run: `mixed-150k-mod`

Server: `26.3-236-f119026 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109869 -> 109869) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 235.880 / 332.004 / 454.847 / 556.726 |
| MSPT mean, ms | 257.408 |
| TPS estimate (1000/p50) | 4.2 |
| heap used / max, MB | 1674.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.16% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9061` |
| 3 | 3.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5943` |
| 4 | 3.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8920` |
| 5 | 2.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.56% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 2.55% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 2.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 2.16% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5789` |
| 11 | 2.07% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8867` |
| 12 | 1.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

