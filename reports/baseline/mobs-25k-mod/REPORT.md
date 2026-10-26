# Baseline run: `mobs-25k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25076 -> 25076) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 149.038 / 214.916 / 274.887 / 285.944 |
| MSPT mean, ms | 153.847 |
| TPS estimate (1000/p50) | 6.7 |
| heap used / max, MB | 1140.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.49% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.58% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 5716` |
| 3 | 2.51% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4364` |
| 4 | 2.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 2.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 6 | 2.07% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.03% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.77% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.75% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 5687` |
| 10 | 1.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 12 | 1.31% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.27% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 1.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.15% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |

