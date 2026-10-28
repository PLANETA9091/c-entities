# Baseline run: `mobs-25k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25056 -> 25056) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 199.303 / 319.339 / 526.830 / 606.818 |
| MSPT mean, ms | 221.023 |
| TPS estimate (1000/p50) | 5.0 |
| heap used / max, MB | 1496.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.94% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.13% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.13% | `java.lang.ref.Reference.get() line: 361` |
| 6 | 1.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 8 | 1.45% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 9 | 1.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 10 | 1.30% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 12 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 14 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |

