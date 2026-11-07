# Baseline run: `mixed-150k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 113253 -> 109564) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 836.264 / 984.729 / 1239.205 / 1403.165 |
| MSPT mean, ms | 868.557 |
| TPS estimate (1000/p50) | 1.2 |
| heap used / max, MB | 2041.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 2.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10830` |
| 2 | 2.95% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 3 | 2.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6967` |
| 4 | 2.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.60% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2690` |
| 6 | 2.11% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 7 | 1.89% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.69% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 9 | 1.51% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.30% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.27% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6204` |
| 15 | 1.13% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |

