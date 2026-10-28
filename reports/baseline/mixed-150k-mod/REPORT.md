# Baseline run: `mixed-150k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109746 -> 109746) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 434.815 / 565.541 / 831.918 / 981.828 |
| MSPT mean, ms | 456.389 |
| TPS estimate (1000/p50) | 2.3 |
| heap used / max, MB | 2079.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.74% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.77% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 4 | 2.62% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 5 | 2.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 7 | 1.56% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 8 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 10 | 1.25% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 11 | 1.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.06% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 1.03% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 15 | 0.99% | `java.lang.ref.Reference.get() line: 361` |

