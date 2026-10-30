# Baseline run: `mixed-150k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109610 -> 109610) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 373.710 / 494.057 / 757.618 / 882.323 |
| MSPT mean, ms | 401.852 |
| TPS estimate (1000/p50) | 2.7 |
| heap used / max, MB | 1859.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8474` |
| 2 | 3.62% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5510` |
| 4 | 3.01% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8615` |
| 5 | 2.66% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.64% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4719` |
| 8 | 1.76% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8421` |
| 9 | 1.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.44% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.42% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5365` |
| 14 | 1.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.11% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |

