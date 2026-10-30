# Baseline run: `mixed-150k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109647 -> 109647) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 297.552 / 412.242 / 585.568 / 712.922 |
| MSPT mean, ms | 325.973 |
| TPS estimate (1000/p50) | 3.4 |
| heap used / max, MB | 1765.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.96% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8622` |
| 2 | 3.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8481` |
| 3 | 3.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5517` |
| 4 | 3.25% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.65% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 2.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4726` |
| 8 | 1.98% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8428` |
| 9 | 1.52% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5372` |
| 10 | 1.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.24% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.01% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |

