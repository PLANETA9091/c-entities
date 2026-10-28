# Baseline run: `mobs-25k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25057 -> 25057) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 159.191 / 348.202 / 640.133 / 801.787 |
| MSPT mean, ms | 200.826 |
| TPS estimate (1000/p50) | 6.3 |
| heap used / max, MB | 2258.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 2.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 4 | 2.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 1.72% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 6 | 1.62% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7960` |
| 7 | 1.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7819` |
| 8 | 1.27% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.09% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4879` |
| 11 | 1.05% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.05% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 1.04% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.02% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2018` |

