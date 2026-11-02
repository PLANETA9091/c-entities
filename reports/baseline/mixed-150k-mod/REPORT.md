# Baseline run: `mixed-150k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109745 -> 109745) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 259.109 / 360.748 / 492.575 / 582.217 |
| MSPT mean, ms | 281.830 |
| TPS estimate (1000/p50) | 3.9 |
| heap used / max, MB | 1907.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.13% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6241` |
| 3 | 3.09% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9949` |
| 4 | 2.72% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 5 | 2.39% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.39% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5473` |
| 8 | 1.82% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.48% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4769` |
| 13 | 1.42% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9433` |
| 14 | 1.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.16% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |

