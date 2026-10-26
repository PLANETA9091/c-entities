# Baseline run: `mixed-150k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100579 -> 100579) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 265.174 / 309.144 / 365.843 / 649.938 |
| MSPT mean, ms | 264.328 |
| TPS estimate (1000/p50) | 3.8 |
| heap used / max, MB | 2734.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.92% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.45% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 4 | 2.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4198` |
| 8 | 1.86% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6454` |
| 9 | 1.81% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.64% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 11 | 1.63% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6343` |
| 12 | 1.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 13 | 1.40% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1426` |
| 15 | 1.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1407` |

