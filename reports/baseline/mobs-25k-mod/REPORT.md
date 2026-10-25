# Baseline run: `mobs-25k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25069 -> 25069) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 148.874 / 266.690 / 343.457 / 410.517 |
| MSPT mean, ms | 162.934 |
| TPS estimate (1000/p50) | 6.7 |
| heap used / max, MB | 1710.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.41% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.60% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 5509` |
| 3 | 2.13% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 2.01% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 1.95% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 6 | 1.68% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4157` |
| 7 | 1.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3817` |
| 9 | 1.50% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.42% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.40% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.28% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 5480` |
| 14 | 1.20% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

