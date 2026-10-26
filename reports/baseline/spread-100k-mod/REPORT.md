# Baseline run: `spread-100k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97353 -> 97353) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 120.776 / 190.480 / 276.057 / 360.982 |
| MSPT mean, ms | 136.271 |
| TPS estimate (1000/p50) | 8.3 |
| heap used / max, MB | 1877.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.38% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 5.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4066` |
| 5 | 2.63% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6320` |
| 6 | 2.21% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6277` |
| 7 | 2.19% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 8 | 2.17% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.80% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.70% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.66% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3921` |
| 12 | 1.54% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 1.21% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6166` |

