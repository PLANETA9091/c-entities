# Baseline run: `mobs-25k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25061 -> 25061) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 152.947 / 284.342 / 356.892 / 428.760 |
| MSPT mean, ms | 170.767 |
| TPS estimate (1000/p50) | 6.5 |
| heap used / max, MB | 1506.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 1.81% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 3 | 1.73% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 4539` |
| 4 | 1.44% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.43% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.18% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4010` |
| 8 | 1.13% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.13% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.13% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.12% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.10% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 4510` |
| 13 | 1.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 14 | 1.02% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.97% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

