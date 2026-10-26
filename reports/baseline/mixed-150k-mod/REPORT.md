# Baseline run: `mixed-150k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100609 -> 100609) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 297.919 / 347.019 / 405.712 / 466.403 |
| MSPT mean, ms | 296.118 |
| TPS estimate (1000/p50) | 3.4 |
| heap used / max, MB | 2436.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.26% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.45% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.99% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6509` |
| 4 | 1.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4200` |
| 5 | 1.78% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.55% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 1.53% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6345` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1428` |
| 10 | 1.28% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 11 | 1.22% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.16% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.96% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 15 | 0.95% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |

