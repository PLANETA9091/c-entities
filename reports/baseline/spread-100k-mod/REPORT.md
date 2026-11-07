# Baseline run: `spread-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97386 -> 97386) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 690.500 / 784.521 / 1047.748 / 1313.883 |
| MSPT mean, ms | 714.373 |
| TPS estimate (1000/p50) | 1.4 |
| heap used / max, MB | 3248.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.11% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 10.01% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2797` |
| 3 | 5.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3250` |
| 4 | 3.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3289` |
| 5 | 2.39% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2488` |
| 6 | 2.26% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2343` |
| 7 | 1.62% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10830` |
| 8 | 1.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6967` |
| 9 | 1.31% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.22% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3263` |
| 12 | 0.91% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3233` |
| 13 | 0.87% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 14 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3247` |
| 15 | 0.84% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

