# Baseline run: `spread-100k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97387 -> 97387) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 174.291 / 250.143 / 321.537 / 433.371 |
| MSPT mean, ms | 183.987 |
| TPS estimate (1000/p50) | 5.7 |
| heap used / max, MB | 1892.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4200` |
| 2 | 3.87% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.30% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6509` |
| 4 | 3.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.02% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 7 | 2.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.94% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.51% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4055` |
| 11 | 1.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.12% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6345` |
| 13 | 1.07% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.00% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 15 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

