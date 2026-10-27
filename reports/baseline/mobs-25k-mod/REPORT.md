# Baseline run: `mobs-25k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25059 -> 25059) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 205.468 / 301.349 / 392.735 / 447.715 |
| MSPT mean, ms | 213.973 |
| TPS estimate (1000/p50) | 4.9 |
| heap used / max, MB | 1708.9 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.85% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.50% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1482` |
| 5 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.44% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7336` |
| 8 | 1.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1463` |
| 9 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7195` |
| 11 | 1.07% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.06% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.94% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 15 | 0.92% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |

