# Baseline run: `mobs-25k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25067 -> 25067) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 309.931 / 555.816 / 702.698 / 965.621 |
| MSPT mean, ms | 359.801 |
| TPS estimate (1000/p50) | 3.2 |
| heap used / max, MB | 1766.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.40% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 1.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 3 | 1.80% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9543` |
| 4 | 1.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.59% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 6 | 1.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9207` |
| 7 | 1.49% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 8 | 1.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.40% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 10 | 1.34% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.13% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.03% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 15 | 0.98% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

