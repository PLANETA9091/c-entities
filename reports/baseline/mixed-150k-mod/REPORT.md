# Baseline run: `mixed-150k-mod`

Server: `26.3-241-5050b0f (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109767 -> 109767) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 469.864 / 567.923 / 738.892 / 852.623 |
| MSPT mean, ms | 488.327 |
| TPS estimate (1000/p50) | 2.1 |
| heap used / max, MB | 1750.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.55% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 2 | 12.92% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 2.83% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10280` |
| 4 | 2.78% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 5 | 2.04% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2410` |
| 6 | 1.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 7 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.82% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1292` |
| 10 | 1.55% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2638` |
| 11 | 1.46% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10916` |
| 12 | 1.44% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2399` |
| 13 | 1.27% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 14 | 1.26% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 1.21% | `java.util.ImmutableCollections$ListItr.next() line: 397` |

