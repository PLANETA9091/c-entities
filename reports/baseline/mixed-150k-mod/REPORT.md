# Baseline run: `mixed-150k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109737 -> 109737) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 266.148 / 369.130 / 564.711 / 666.237 |
| MSPT mean, ms | 289.424 |
| TPS estimate (1000/p50) | 3.8 |
| heap used / max, MB | 2035.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.26% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.00% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6273` |
| 4 | 2.92% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10150` |
| 5 | 2.00% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 1.95% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.92% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5505` |
| 8 | 1.74% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.73% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.69% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.61% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9548` |
| 14 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.13% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |

