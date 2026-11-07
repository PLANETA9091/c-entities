# Baseline run: `spread-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97444 -> 97444) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 627.054 / 896.234 / 1224.807 / 1463.078 |
| MSPT mean, ms | 673.077 |
| TPS estimate (1000/p50) | 1.6 |
| heap used / max, MB | 3191.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.18% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2831` |
| 2 | 10.64% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3366` |
| 4 | 2.36% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 393` |
| 5 | 2.29% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1312` |
| 6 | 1.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10935` |
| 7 | 1.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 8 | 1.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3310` |
| 10 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 0.95% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 0.91% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3340` |
| 13 | 0.91% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 14 | 0.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 0.82% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

