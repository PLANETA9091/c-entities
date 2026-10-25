# Baseline run: `mobs-25k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25068 -> 25068) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 144.911 / 272.078 / 356.041 / 378.938 |
| MSPT mean, ms | 161.907 |
| TPS estimate (1000/p50) | 6.9 |
| heap used / max, MB | 1614.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.79% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.37% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 4065` |
| 3 | 2.26% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 1.87% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 4094` |
| 5 | 1.75% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 7 | 1.59% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3624` |
| 8 | 1.58% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.47% | `java.util.EnumMap.get(Object) line: 248` |
| 10 | 1.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.32% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3284` |
| 13 | 1.09% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.08% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 15 | 1.04% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

