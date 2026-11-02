# Baseline run: `mobs-25k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25051 -> 25051) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 285.213 / 466.678 / 572.845 / 627.060 |
| MSPT mean, ms | 322.027 |
| TPS estimate (1000/p50) | 3.5 |
| heap used / max, MB | 1977.9 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 2.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 2 | 2.31% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.25% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9949` |
| 4 | 2.07% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 2.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 8 | 1.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9580` |
| 9 | 1.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.40% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.13% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 14 | 0.97% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 15 | 0.80% | `java.lang.ref.Reference.get() line: 361` |

