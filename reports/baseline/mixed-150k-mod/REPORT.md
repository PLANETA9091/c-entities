# Baseline run: `mixed-150k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109802 -> 109802) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 287.995 / 399.814 / 538.782 / 626.314 |
| MSPT mean, ms | 311.693 |
| TPS estimate (1000/p50) | 3.5 |
| heap used / max, MB | 1661.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.80% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.04% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8463` |
| 3 | 2.81% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.45% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.43% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8657` |
| 6 | 2.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5552` |
| 7 | 2.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 2.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8516` |
| 9 | 2.17% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 2.14% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5407` |
| 11 | 2.14% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.80% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 14 | 1.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4761` |

