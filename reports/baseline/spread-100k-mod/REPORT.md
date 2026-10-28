# Baseline run: `spread-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97402 -> 97402) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 183.061 / 319.296 / 475.487 / 660.125 |
| MSPT mean, ms | 210.838 |
| TPS estimate (1000/p50) | 5.5 |
| heap used / max, MB | 2336.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.58% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 2 | 3.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 3 | 3.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 5 | 2.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.35% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 8 | 1.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.92% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 10 | 1.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 11 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.21% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 14 | 1.14% | `java.lang.ref.Reference.get() line: 361` |
| 15 | 1.07% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |

