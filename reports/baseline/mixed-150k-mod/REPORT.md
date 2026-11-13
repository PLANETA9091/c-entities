# Baseline run: `mixed-150k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 103480 -> 103446) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 163.090 / 277.871 / 397.003 / 489.220 |
| MSPT mean, ms | 193.070 |
| TPS estimate (1000/p50) | 6.1 |
| heap used / max, MB | 2150.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7356` |
| 2 | 4.96% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 4.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11978` |
| 4 | 4.24% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.62% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 7 | 2.24% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 8 | 1.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.66% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 11 | 1.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.35% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 13 | 1.33% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.14% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 15 | 1.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7377` |

