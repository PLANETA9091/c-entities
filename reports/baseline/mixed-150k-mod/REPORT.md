# Baseline run: `mixed-150k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110432 -> 110400) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 152.955 / 254.976 / 398.155 / 498.873 |
| MSPT mean, ms | 180.381 |
| TPS estimate (1000/p50) | 6.5 |
| heap used / max, MB | 2352.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7769` |
| 2 | 5.79% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 4.62% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 4.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12392` |
| 5 | 4.33% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 6 | 2.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 7 | 2.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.86% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 10 | 1.83% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 11 | 1.65% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.25% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 1.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

