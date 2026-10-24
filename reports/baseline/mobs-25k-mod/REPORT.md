# Baseline run: `mobs-25k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25060 -> 25060) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 191.611 / 319.114 / 395.926 / 453.949 |
| MSPT mean, ms | 205.309 |
| TPS estimate (1000/p50) | 5.2 |
| heap used / max, MB | 1411.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 3 | 2.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 4 | 1.85% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 5 | 1.77% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1358` |
| 6 | 1.73% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.73% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 8 | 1.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.35% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.34% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 11 | 1.28% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 1.13% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.09% | `net.minecraft.util.Mth.wrapDegrees(float) line: 226` |

