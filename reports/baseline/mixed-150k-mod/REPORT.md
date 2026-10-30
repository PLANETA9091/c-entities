# Baseline run: `mixed-150k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109829 -> 109829) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 319.793 / 444.557 / 645.593 / 728.430 |
| MSPT mean, ms | 350.456 |
| TPS estimate (1000/p50) | 3.1 |
| heap used / max, MB | 1852.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8492` |
| 2 | 3.24% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5528` |
| 4 | 2.78% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.50% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.42% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8633` |
| 7 | 2.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4737` |
| 8 | 1.85% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8439` |
| 9 | 1.67% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.58% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5383` |
| 12 | 1.44% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.07% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |

