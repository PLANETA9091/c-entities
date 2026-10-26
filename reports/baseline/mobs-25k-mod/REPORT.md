# Baseline run: `mobs-25k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25064 -> 25064) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 156.371 / 271.916 / 328.013 / 399.652 |
| MSPT mean, ms | 170.975 |
| TPS estimate (1000/p50) | 6.4 |
| heap used / max, MB | 1376.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.42% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.49% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.37% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6354` |
| 4 | 1.88% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 1.77% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.51% | `java.util.EnumMap.get(Object) line: 248` |
| 9 | 1.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 11 | 1.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 12 | 1.21% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6320` |
| 13 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.16% | `dev.planeta9091.centities.CeHooks.gsMirrorTickRunning(GoalSelector, boolean) line: 4745` |
| 15 | 1.14% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

