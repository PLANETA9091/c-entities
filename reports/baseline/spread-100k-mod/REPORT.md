# Baseline run: `spread-100k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97334 -> 97334) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 178.159 / 267.415 / 350.114 / 456.012 |
| MSPT mean, ms | 196.138 |
| TPS estimate (1000/p50) | 5.6 |
| heap used / max, MB | 2441.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.84% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.get(VarHandle, Object) line: 92` |
| 2 | 4.34% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 4.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3097` |
| 4 | 4.02% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2962` |
| 5 | 3.65% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 3659` |
| 6 | 3.21% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 7 | 2.61% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2972` |
| 8 | 2.41% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1467` |
| 9 | 1.99% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.45% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 3633` |
| 13 | 1.42% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 1.21% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 15 | 1.17% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

