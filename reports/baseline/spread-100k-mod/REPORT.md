# Baseline run: `spread-100k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97355 -> 97355) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 162.835 / 248.401 / 325.615 / 393.051 |
| MSPT mean, ms | 175.062 |
| TPS estimate (1000/p50) | 6.1 |
| heap used / max, MB | 2569.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.51% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.71% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6497` |
| 3 | 3.61% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6444` |
| 4 | 3.50% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4198` |
| 6 | 2.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 2.27% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.88% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4053` |
| 10 | 1.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.21% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 12 | 1.17% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.12% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4376` |
| 15 | 1.02% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |

