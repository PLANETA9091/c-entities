# Baseline run: `mobs-25k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25065 -> 25065) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 152.138 / 237.428 / 359.440 / 418.845 |
| MSPT mean, ms | 168.656 |
| TPS estimate (1000/p50) | 6.6 |
| heap used / max, MB | 1378.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.05% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.76% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.70% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.10% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 1.78% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7862` |
| 6 | 1.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7721` |
| 7 | 1.66% | `java.util.EnumMap.get(Object) line: 248` |
| 8 | 1.58% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1943` |
| 9 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.39% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 11 | 1.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.39% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 1.31% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 14 | 1.06% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6755` |

