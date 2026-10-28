# Baseline run: `mobs-25k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25070 -> 25070) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 239.448 / 469.643 / 668.124 / 798.936 |
| MSPT mean, ms | 277.417 |
| TPS estimate (1000/p50) | 4.2 |
| heap used / max, MB | 1589.9 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.24% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.34% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.31% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 1.93% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1876` |
| 5 | 1.69% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7776` |
| 6 | 1.68% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7843` |
| 7 | 1.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1857` |
| 9 | 1.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.13% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 12 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7635` |
| 14 | 0.90% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.87% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

