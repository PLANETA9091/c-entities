# Baseline run: `spread-100k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97310 -> 97310) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 127.808 / 211.825 / 296.992 / 418.761 |
| MSPT mean, ms | 146.218 |
| TPS estimate (1000/p50) | 7.8 |
| heap used / max, MB | 2680.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.04% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3324` |
| 3 | 3.54% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1494` |
| 4 | 3.44% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4210` |
| 5 | 2.07% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 1.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3460` |
| 7 | 1.92% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.84% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 9 | 1.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.22% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3800` |
| 12 | 1.16% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.06% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 0.99% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 15 | 0.98% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3314` |

