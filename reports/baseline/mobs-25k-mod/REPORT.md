# Baseline run: `mobs-25k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25045 -> 25045) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 180.514 / 279.503 / 351.713 / 379.457 |
| MSPT mean, ms | 192.296 |
| TPS estimate (1000/p50) | 5.5 |
| heap used / max, MB | 1850.8 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.80% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 1.72% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6970` |
| 5 | 1.70% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6921` |
| 6 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 9 | 1.07% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 0.98% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 12 | 0.95% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 0.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 0.86% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 15 | 0.85% | `java.util.ImmutableCollections$ListItr.next() line: 397` |

