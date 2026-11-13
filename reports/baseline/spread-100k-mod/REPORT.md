# Baseline run: `spread-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 84860 -> 84731) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 179.878 / 253.167 / 398.916 / 450.522 |
| MSPT mean, ms | 198.913 |
| TPS estimate (1000/p50) | 5.6 |
| heap used / max, MB | 1978.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.30% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 5.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3461` |
| 3 | 3.58% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3405` |
| 4 | 2.93% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11978` |
| 5 | 2.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7347` |
| 6 | 2.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3425` |
| 7 | 2.31% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 8 | 2.13% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.92% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3443` |
| 10 | 1.57% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3422` |
| 11 | 1.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3435` |
| 12 | 1.47% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 13 | 1.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.45% | `java.util.ArrayList.<init>(int) line: 157` |
| 15 | 1.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |

