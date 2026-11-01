# Baseline run: `spread-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97440 -> 97440) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 297.943 / 466.374 / 623.468 / 783.074 |
| MSPT mean, ms | 332.505 |
| TPS estimate (1000/p50) | 3.4 |
| heap used / max, MB | 1845.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.22% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9129` |
| 2 | 3.26% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.99% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8935` |
| 4 | 2.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6011` |
| 7 | 2.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8988` |
| 8 | 2.03% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5857` |
| 9 | 1.79% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 10 | 1.73% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.66% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 12 | 1.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.22% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 1.17% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |

