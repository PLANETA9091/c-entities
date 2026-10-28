# Baseline run: `mixed-150k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109710 -> 109706) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 557.319 / 711.029 / 1008.653 / 1133.656 |
| MSPT mean, ms | 569.349 |
| TPS estimate (1000/p50) | 1.8 |
| heap used / max, MB | 1874.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 16.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.45% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 5 | 2.22% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.16% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 7 | 1.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 8 | 1.27% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.23% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 10 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.18% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 0.99% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 0.92% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 0.90% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |

