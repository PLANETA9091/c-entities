# Baseline run: `spread-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97402 -> 97402) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 170.003 / 290.211 / 468.608 / 521.578 |
| MSPT mean, ms | 194.304 |
| TPS estimate (1000/p50) | 5.9 |
| heap used / max, MB | 2316.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.76% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 3 | 3.30% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7685` |
| 4 | 3.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 5 | 2.42% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 2.27% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.61% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4459` |
| 11 | 1.55% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 12 | 1.17% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.08% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 0.81% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |

