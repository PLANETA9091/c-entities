# Baseline run: `mixed-150k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100574 -> 100574) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 264.646 / 313.202 / 363.636 / 419.413 |
| MSPT mean, ms | 270.567 |
| TPS estimate (1000/p50) | 3.8 |
| heap used / max, MB | 1782.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.26% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3860` |
| 6 | 2.18% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 7 | 2.11% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 5687` |
| 8 | 1.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4024` |
| 9 | 1.81% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5646` |
| 10 | 1.68% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.64% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.51% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 14 | 1.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.33% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 5550` |

