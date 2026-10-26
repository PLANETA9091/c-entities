# Baseline run: `spread-100k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97409 -> 97409) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 140.376 / 212.423 / 279.757 / 398.413 |
| MSPT mean, ms | 152.234 |
| TPS estimate (1000/p50) | 7.1 |
| heap used / max, MB | 2706.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.26% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.76% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.40% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6703` |
| 4 | 3.33% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6650` |
| 5 | 2.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 6 | 2.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.79% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.73% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4058` |
| 10 | 1.08% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6539` |
| 11 | 1.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 0.99% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 0.94% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4381` |

