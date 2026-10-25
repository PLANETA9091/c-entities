# Baseline run: `mixed-150k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100078 -> 100050) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 357.246 / 406.604 / 465.734 / 522.271 |
| MSPT mean, ms | 365.389 |
| TPS estimate (1000/p50) | 2.8 |
| heap used / max, MB | 2874.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.07% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.50% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 3.53% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 4394` |
| 4 | 3.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 3.23% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1495` |
| 6 | 2.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 2.31% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 8 | 2.14% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.97% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3467` |
| 10 | 1.68% | `java.util.HashMap.getNode(Object) line: 586` |
| 11 | 1.58% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 12 | 1.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3613` |
| 13 | 1.39% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.39% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 15 | 1.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

