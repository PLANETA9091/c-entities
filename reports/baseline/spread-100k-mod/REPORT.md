# Baseline run: `spread-100k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97375 -> 97375) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 123.129 / 200.134 / 268.275 / 374.462 |
| MSPT mean, ms | 138.795 |
| TPS estimate (1000/p50) | 8.1 |
| heap used / max, MB | 2634.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.94% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5439` |
| 3 | 3.76% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3681` |
| 4 | 2.82% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 5480` |
| 5 | 2.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.94% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3817` |
| 9 | 1.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.33% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4157` |
| 11 | 1.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.15% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 13 | 1.15% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.05% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 5343` |
| 15 | 1.03% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |

