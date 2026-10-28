# Baseline run: `mixed-150k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109724 -> 109724) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 547.416 / 708.119 / 996.386 / 1166.865 |
| MSPT mean, ms | 565.093 |
| TPS estimate (1000/p50) | 1.8 |
| heap used / max, MB | 1762.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 18.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.43% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 3 | 2.32% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 6 | 2.27% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 8 | 1.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 11 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.04% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 13 | 1.03% | `java.lang.ref.Reference.get() line: 361` |
| 14 | 0.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 0.83% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |

