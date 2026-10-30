# Baseline run: `mixed-150k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109727 -> 109727) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 254.424 / 375.297 / 608.014 / 723.697 |
| MSPT mean, ms | 291.375 |
| TPS estimate (1000/p50) | 3.9 |
| heap used / max, MB | 2011.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.71% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8572` |
| 2 | 4.67% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 4.29% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.13% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8431` |
| 7 | 2.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5467` |
| 8 | 2.46% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.15% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8378` |
| 10 | 2.09% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5322` |
| 11 | 1.78% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4676` |
| 13 | 1.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.40% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.39% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |

