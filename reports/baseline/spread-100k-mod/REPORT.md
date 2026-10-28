# Baseline run: `spread-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97488 -> 97488) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 163.069 / 285.153 / 429.505 / 548.120 |
| MSPT mean, ms | 188.604 |
| TPS estimate (1000/p50) | 6.1 |
| heap used / max, MB | 2768.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7721` |
| 3 | 3.07% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7862` |
| 4 | 2.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4781` |
| 5 | 2.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.34% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.15% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 2.12% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4636` |
| 10 | 1.86% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7668` |
| 11 | 1.82% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.05% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.03% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |

