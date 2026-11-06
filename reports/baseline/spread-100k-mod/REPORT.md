# Baseline run: `spread-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97439 -> 97439) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 437.900 / 581.073 / 745.010 / 908.280 |
| MSPT mean, ms | 459.530 |
| TPS estimate (1000/p50) | 2.3 |
| heap used / max, MB | 2104.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.00% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 2 | 4.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 3 | 4.57% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3100` |
| 4 | 3.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3138` |
| 5 | 2.92% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 11028` |
| 6 | 2.62% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10392` |
| 7 | 2.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.15% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 10 | 1.77% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3112` |
| 11 | 1.76% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3083` |
| 13 | 0.97% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 0.93% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 15 | 0.91% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |

