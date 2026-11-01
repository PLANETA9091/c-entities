# Baseline run: `mobs-25k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25063 -> 25063) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 276.603 / 492.904 / 682.377 / 786.007 |
| MSPT mean, ms | 321.267 |
| TPS estimate (1000/p50) | 3.6 |
| heap used / max, MB | 1211.1 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.20% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 2.01% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 1.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5943` |
| 6 | 1.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.59% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 8 | 1.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 9 | 1.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 10 | 1.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8920` |
| 11 | 1.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 12 | 1.22% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8867` |
| 13 | 1.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.17% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5789` |
| 15 | 1.16% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |

