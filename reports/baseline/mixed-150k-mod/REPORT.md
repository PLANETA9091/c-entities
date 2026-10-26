# Baseline run: `mixed-150k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100572 -> 100572) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 295.179 / 351.685 / 403.081 / 452.007 |
| MSPT mean, ms | 300.646 |
| TPS estimate (1000/p50) | 3.4 |
| heap used / max, MB | 2371.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.53% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6660` |
| 5 | 1.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |
| 6 | 1.78% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.75% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1430` |
| 8 | 1.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.31% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6496` |
| 12 | 1.23% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 13 | 1.21% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6607` |
| 14 | 1.12% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.01% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |

