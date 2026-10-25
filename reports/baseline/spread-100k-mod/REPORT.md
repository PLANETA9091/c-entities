# Baseline run: `spread-100k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97412 -> 97412) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 159.771 / 249.567 / 335.219 / 474.303 |
| MSPT mean, ms | 175.400 |
| TPS estimate (1000/p50) | 6.3 |
| heap used / max, MB | 1812.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.43% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 2 | 3.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3806` |
| 4 | 3.13% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.40% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3624` |
| 6 | 2.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.99% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.65% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3625` |
| 10 | 1.25% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5422` |
| 11 | 1.24% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.17% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.16% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.15% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 15 | 0.97% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 5326` |

