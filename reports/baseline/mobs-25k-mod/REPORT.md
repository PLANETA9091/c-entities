# Baseline run: `mobs-25k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25066 -> 25066) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 160.363 / 309.406 / 560.547 / 752.139 |
| MSPT mean, ms | 192.423 |
| TPS estimate (1000/p50) | 6.2 |
| heap used / max, MB | 1380.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 2.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.54% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 3 | 2.30% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 2.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4886` |
| 7 | 1.69% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7826` |
| 9 | 1.48% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 11 | 1.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 12 | 1.11% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.10% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2018` |
| 14 | 1.06% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

