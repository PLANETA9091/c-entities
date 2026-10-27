# Baseline run: `spread-100k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97413 -> 97413) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 145.246 / 220.144 / 305.722 / 423.297 |
| MSPT mean, ms | 160.165 |
| TPS estimate (1000/p50) | 6.9 |
| heap used / max, MB | 2762.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.99% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7317` |
| 2 | 3.65% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7176` |
| 4 | 3.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4236` |
| 5 | 2.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.20% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.99% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.87% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.76% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4091` |
| 11 | 1.69% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7123` |
| 12 | 1.02% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 0.90% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.85% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 15 | 0.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

