# Baseline run: `spread-100k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97355 -> 97355) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 145.065 / 217.957 / 310.275 / 379.387 |
| MSPT mean, ms | 157.306 |
| TPS estimate (1000/p50) | 6.9 |
| heap used / max, MB | 2221.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.84% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7195` |
| 3 | 3.62% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7336` |
| 4 | 2.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4255` |
| 6 | 2.43% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 2.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.00% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.78% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7142` |
| 10 | 1.78% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.73% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4110` |
| 12 | 1.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 0.93% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 15 | 0.87% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |

