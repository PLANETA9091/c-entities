# Baseline run: `mixed-150k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109725 -> 109725) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 234.705 / 326.214 / 463.568 / 522.233 |
| MSPT mean, ms | 255.495 |
| TPS estimate (1000/p50) | 4.3 |
| heap used / max, MB | 1616.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.59% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.30% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6163` |
| 4 | 3.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9204` |
| 5 | 2.77% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.63% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9345` |
| 7 | 2.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 2.44% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.32% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 2.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 2.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9118` |
| 12 | 2.05% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 6009` |
| 13 | 1.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.41% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 15 | 1.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

