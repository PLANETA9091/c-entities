# Baseline run: `spread-100k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97530 -> 97530) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 151.067 / 250.557 / 378.927 / 487.422 |
| MSPT mean, ms | 172.021 |
| TPS estimate (1000/p50) | 6.6 |
| heap used / max, MB | 2291.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.49% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8125` |
| 3 | 3.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5020` |
| 4 | 3.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7984` |
| 5 | 2.85% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.68% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7931` |
| 8 | 2.06% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4875` |
| 9 | 1.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.75% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.70% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.13% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

