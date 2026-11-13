# Baseline run: `spread-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 84852 -> 84723) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 190.329 / 263.889 / 390.554 / 478.226 |
| MSPT mean, ms | 208.302 |
| TPS estimate (1000/p50) | 5.3 |
| heap used / max, MB | 2110.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.14% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 5.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3461` |
| 3 | 3.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3405` |
| 4 | 2.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7347` |
| 5 | 2.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11978` |
| 6 | 2.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 7 | 2.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3418` |
| 9 | 1.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3443` |
| 10 | 1.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3435` |
| 11 | 1.80% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3407` |
| 12 | 1.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3425` |
| 13 | 1.58% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3422` |
| 14 | 1.53% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 15 | 1.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |

