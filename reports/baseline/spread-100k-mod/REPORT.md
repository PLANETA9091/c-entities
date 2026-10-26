# Baseline run: `spread-100k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97379 -> 97379) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 150.812 / 229.444 / 298.020 / 426.213 |
| MSPT mean, ms | 163.808 |
| TPS estimate (1000/p50) | 6.6 |
| heap used / max, MB | 1988.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.21% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.68% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 2.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4198` |
| 5 | 2.37% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6507` |
| 6 | 2.30% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.77% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.62% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6454` |
| 9 | 1.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.26% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6343` |
| 12 | 1.13% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 0.99% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 14 | 0.94% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4053` |
| 15 | 0.93% | `org.spigotmc.WatchdogThread.run() line: 72` |

