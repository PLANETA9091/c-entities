# Baseline run: `mixed-150k-mod`

Server: `26.3-242-fd28d65 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109694 -> 109694) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 284.937 / 371.546 / 474.478 / 616.173 |
| MSPT mean, ms | 304.774 |
| TPS estimate (1000/p50) | 3.5 |
| heap used / max, MB | 2072.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.11% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 2 | 5.87% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 3 | 4.31% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.52% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10314` |
| 5 | 3.25% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 2.61% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10950` |
| 7 | 1.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 8 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.73% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.59% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.52% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.25% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.17% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.17% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

