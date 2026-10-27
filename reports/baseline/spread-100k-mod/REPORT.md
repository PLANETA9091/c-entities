# Baseline run: `spread-100k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97331 -> 97331) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 136.685 / 214.757 / 331.975 / 389.271 |
| MSPT mean, ms | 153.001 |
| TPS estimate (1000/p50) | 7.3 |
| heap used / max, MB | 2215.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.65% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.50% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 3 | 3.19% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7603` |
| 4 | 3.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 5 | 2.81% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 7 | 2.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 2.35% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 9 | 2.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 2.05% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4377` |
| 11 | 1.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 13 | 1.32% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.27% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.02% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |

