# Baseline run: `mixed-150k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109749 -> 109749) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 300.771 / 418.189 / 587.548 / 850.514 |
| MSPT mean, ms | 331.404 |
| TPS estimate (1000/p50) | 3.3 |
| heap used / max, MB | 1755.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.71% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 4.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8691` |
| 3 | 3.84% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8832` |
| 4 | 3.21% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5727` |
| 6 | 2.77% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4936` |
| 8 | 1.71% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8638` |
| 9 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.41% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5582` |
| 11 | 1.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.18% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.15% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 1.04% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

