# Baseline run: `mixed-150k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109779 -> 109779) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 232.451 / 337.471 / 506.172 / 558.436 |
| MSPT mean, ms | 260.141 |
| TPS estimate (1000/p50) | 4.3 |
| heap used / max, MB | 1926.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6351` |
| 2 | 2.97% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10239` |
| 3 | 2.78% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.56% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 2.29% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 6 | 2.02% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.69% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9637` |
| 8 | 1.64% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 9 | 1.40% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getAcquire(VarHandle, Object) line: 113` |
| 10 | 1.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.30% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |

