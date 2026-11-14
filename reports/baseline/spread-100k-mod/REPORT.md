# Baseline run: `spread-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88797 -> 88662) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 152.257 / 220.354 / 335.892 / 447.187 |
| MSPT mean, ms | 164.852 |
| TPS estimate (1000/p50) | 6.6 |
| heap used / max, MB | 2944.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3698` |
| 2 | 5.98% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.76% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3642` |
| 4 | 3.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12449` |
| 6 | 2.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3659` |
| 7 | 2.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7064` |
| 8 | 2.05% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3672` |
| 9 | 2.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7817` |
| 10 | 1.83% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3656` |
| 11 | 1.72% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3671` |
| 13 | 1.45% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.24% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |

