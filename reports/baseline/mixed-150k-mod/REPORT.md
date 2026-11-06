# Baseline run: `mixed-150k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109731 -> 109731) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 424.015 / 529.670 / 721.455 / 840.928 |
| MSPT mean, ms | 449.764 |
| TPS estimate (1000/p50) | 2.4 |
| heap used / max, MB | 1853.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 15.70% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 2 | 9.97% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.84% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 4 | 3.09% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10392` |
| 5 | 2.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.16% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 11028` |
| 7 | 1.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 8 | 1.70% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 9 | 1.13% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.12% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.07% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.05% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2399` |
| 14 | 0.97% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 0.94% | `java.util.ImmutableCollections$ListItr.next() line: 397` |

