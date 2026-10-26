# Baseline run: `mobs-25k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25061 -> 25061) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 147.485 / 221.034 / 291.077 / 333.098 |
| MSPT mean, ms | 159.052 |
| TPS estimate (1000/p50) | 6.8 |
| heap used / max, MB | 1541.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.91% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.67% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 3 | 2.16% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 2.05% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6307` |
| 5 | 1.99% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6495` |
| 6 | 1.97% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.80% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.67% | `java.util.EnumMap.get(Object) line: 248` |
| 10 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.46% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 12 | 1.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.27% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 14 | 1.23% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 15 | 1.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |

