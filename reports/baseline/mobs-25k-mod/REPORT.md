# Baseline run: `mobs-25k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25057 -> 25057) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 226.372 / 403.600 / 616.185 / 762.914 |
| MSPT mean, ms | 260.227 |
| TPS estimate (1000/p50) | 4.4 |
| heap used / max, MB | 1401.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.41% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 1.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 5 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.24% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.19% | `java.util.EnumMap.get(Object) line: 248` |
| 8 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 10 | 1.08% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 11 | 1.06% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 12 | 1.05% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7685` |
| 13 | 1.02% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.97% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.92% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |

