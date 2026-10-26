# Baseline run: `mobs-25k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25058 -> 25058) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 174.544 / 244.184 / 313.368 / 383.440 |
| MSPT mean, ms | 181.167 |
| TPS estimate (1000/p50) | 5.7 |
| heap used / max, MB | 1520.1 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.16% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.77% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1411` |
| 4 | 2.22% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6633` |
| 5 | 2.18% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.88% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.66% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6584` |
| 9 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], float[], Mob) line: 5906` |
| 11 | 1.45% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1430` |
| 13 | 1.21% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 14 | 1.19% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 15 | 1.16% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |

