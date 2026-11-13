# Baseline run: `mixed-150k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 103439 -> 103408) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 162.695 / 274.787 / 416.960 / 505.711 |
| MSPT mean, ms | 190.513 |
| TPS estimate (1000/p50) | 6.1 |
| heap used / max, MB | 2152.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7410` |
| 2 | 5.81% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 4.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12033` |
| 4 | 4.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 4.37% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 6 | 2.83% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 7 | 1.89% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 8 | 1.79% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 9 | 1.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.63% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.41% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.32% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 15 | 1.24% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

