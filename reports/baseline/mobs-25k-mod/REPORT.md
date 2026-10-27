# Baseline run: `mobs-25k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25065 -> 25065) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 161.678 / 270.227 / 451.437 / 585.999 |
| MSPT mean, ms | 182.667 |
| TPS estimate (1000/p50) | 6.2 |
| heap used / max, MB | 1501.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.34% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 1.91% | `java.util.EnumMap.get(Object) line: 248` |
| 5 | 1.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.44% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7511` |
| 9 | 1.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.16% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4571` |
| 11 | 1.14% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 12 | 1.09% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 13 | 1.03% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 14 | 0.96% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 15 | 0.94% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |

