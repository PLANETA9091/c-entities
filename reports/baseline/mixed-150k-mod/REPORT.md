# Baseline run: `mixed-150k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 103476 -> 103438) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 159.302 / 271.484 / 416.222 / 541.580 |
| MSPT mean, ms | 188.427 |
| TPS estimate (1000/p50) | 6.3 |
| heap used / max, MB | 2142.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7356` |
| 2 | 6.36% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 4.59% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 4.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11978` |
| 5 | 3.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.10% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 7 | 2.03% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 8 | 1.85% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 9 | 1.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.81% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.57% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.54% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 13 | 1.30% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.21% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.20% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

