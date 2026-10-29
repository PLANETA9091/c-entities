# Baseline run: `mobs-25k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25061 -> 25061) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 156.106 / 295.035 / 543.399 / 593.492 |
| MSPT mean, ms | 189.780 |
| TPS estimate (1000/p50) | 6.4 |
| heap used / max, MB | 1918.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.90% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 2.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 1.94% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5092` |
| 8 | 1.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8056` |
| 9 | 1.43% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8197` |
| 10 | 1.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 11 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.19% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 13 | 1.09% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 14 | 1.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.96% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

