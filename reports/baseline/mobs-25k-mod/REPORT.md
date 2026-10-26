# Baseline run: `mobs-25k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25060 -> 25060) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 197.086 / 288.405 / 365.743 / 411.564 |
| MSPT mean, ms | 204.877 |
| TPS estimate (1000/p50) | 5.1 |
| heap used / max, MB | 1303.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.92% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.55% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 1.54% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6558` |
| 5 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.36% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], float[], Mob) line: 5885` |
| 8 | 1.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.10% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 10 | 1.03% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6509` |
| 11 | 1.00% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 0.99% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 0.95% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.94% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 15 | 0.94% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |

