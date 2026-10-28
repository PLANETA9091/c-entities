# Baseline run: `spread-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97340 -> 97340) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 167.758 / 301.585 / 456.242 / 639.317 |
| MSPT mean, ms | 195.280 |
| TPS estimate (1000/p50) | 6.0 |
| heap used / max, MB | 2525.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7656` |
| 3 | 2.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 2.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4716` |
| 5 | 2.24% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.92% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.78% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.63% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7797` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4571` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7603` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4007` |
| 13 | 1.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.00% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |

