# Baseline run: `mobs-25k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25064 -> 25064) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 137.660 / 199.520 / 249.496 / 295.526 |
| MSPT mean, ms | 144.560 |
| TPS estimate (1000/p50) | 7.3 |
| heap used / max, MB | 1570.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 2.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.20% | `java.util.EnumMap.get(Object) line: 248` |
| 5 | 2.07% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 2.05% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1445` |
| 7 | 1.85% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7317` |
| 8 | 1.77% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7176` |
| 10 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.21% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 12 | 1.16% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 13 | 1.09% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 0.96% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 15 | 0.96% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4091` |

