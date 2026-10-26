# Baseline run: `mobs-25k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25063 -> 25063) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 178.799 / 261.460 / 350.533 / 405.895 |
| MSPT mean, ms | 191.138 |
| TPS estimate (1000/p50) | 5.6 |
| heap used / max, MB | 1602.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.40% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 1.83% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6703` |
| 5 | 1.50% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 1.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 8 | 1.21% | `java.util.EnumMap.get(Object) line: 248` |
| 9 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.12% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.03% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 0.92% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 13 | 0.91% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 14 | 0.85% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 0.85% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6539` |

