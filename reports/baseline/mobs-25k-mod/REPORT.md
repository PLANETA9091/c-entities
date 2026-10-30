# Baseline run: `mobs-25k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25063 -> 25063) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 258.635 / 385.781 / 518.943 / 565.425 |
| MSPT mean, ms | 286.652 |
| TPS estimate (1000/p50) | 3.9 |
| heap used / max, MB | 1657.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.25% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.44% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.37% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 2.06% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5318` |
| 6 | 1.55% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 9 | 1.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 10 | 1.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8282` |
| 11 | 1.23% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 12 | 1.23% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8229` |
| 14 | 1.07% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

