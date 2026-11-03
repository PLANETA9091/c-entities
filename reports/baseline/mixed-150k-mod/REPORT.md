# Baseline run: `mixed-150k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109733 -> 109733) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 267.046 / 384.079 / 570.206 / 656.580 |
| MSPT mean, ms | 294.409 |
| TPS estimate (1000/p50) | 3.7 |
| heap used / max, MB | 1999.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 2 | 3.59% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10380` |
| 3 | 3.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.48% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 2.05% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 1.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 7 | 1.72% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 9 | 1.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.39% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9778` |
| 12 | 1.29% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.11% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 0.93% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |

