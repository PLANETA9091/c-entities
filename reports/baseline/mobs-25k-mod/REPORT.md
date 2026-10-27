# Baseline run: `mobs-25k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25073 -> 25073) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 235.003 / 447.020 / 668.879 / 743.614 |
| MSPT mean, ms | 275.789 |
| TPS estimate (1000/p50) | 4.3 |
| heap used / max, MB | 1651.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 1.87% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 1.86% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.80% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7357` |
| 6 | 1.61% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 7 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.25% | `java.util.EnumMap.get(Object) line: 248` |
| 9 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.17% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.13% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.08% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4276` |
| 13 | 1.08% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.04% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 15 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

