# Baseline run: `mixed-150k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109753 -> 109753) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 253.024 / 354.512 / 534.928 / 673.640 |
| MSPT mean, ms | 278.237 |
| TPS estimate (1000/p50) | 4.0 |
| heap used / max, MB | 1863.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.53% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9116` |
| 3 | 3.96% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.54% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9257` |
| 6 | 3.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6139` |
| 8 | 2.08% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5985` |
| 9 | 2.07% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9063` |
| 10 | 1.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5403` |
| 12 | 1.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.19% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 15 | 1.15% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |

