# Baseline run: `spread-100k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97418 -> 97418) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 151.666 / 231.511 / 314.945 / 385.813 |
| MSPT mean, ms | 164.595 |
| TPS estimate (1000/p50) | 6.6 |
| heap used / max, MB | 2613.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.25% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.73% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6473` |
| 4 | 2.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4197` |
| 5 | 2.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.56% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.48% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 8 | 2.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6309` |
| 10 | 1.22% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4052` |
| 11 | 1.19% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.18% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.17% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 1.00% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.97% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |

