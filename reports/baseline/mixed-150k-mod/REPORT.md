# Baseline run: `mixed-150k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109777 -> 109777) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 262.118 / 374.595 / 499.650 / 581.446 |
| MSPT mean, ms | 288.775 |
| TPS estimate (1000/p50) | 3.8 |
| heap used / max, MB | 1645.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.84% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8933` |
| 2 | 3.33% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.17% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9127` |
| 4 | 3.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.76% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6009` |
| 7 | 2.43% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.29% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5855` |
| 9 | 2.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8986` |
| 10 | 1.44% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 12 | 1.30% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5273` |
| 14 | 1.21% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 15 | 1.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

