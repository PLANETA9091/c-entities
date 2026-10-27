# Baseline run: `mobs-25k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25055 -> 25055) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 138.499 / 201.524 / 256.946 / 298.602 |
| MSPT mean, ms | 145.774 |
| TPS estimate (1000/p50) | 7.2 |
| heap used / max, MB | 1749.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.62% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6972` |
| 4 | 2.29% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 6 | 1.95% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.83% | `java.util.EnumMap.get(Object) line: 248` |
| 8 | 1.51% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6923` |
| 9 | 1.41% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 10 | 1.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.22% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 1.16% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 14 | 1.05% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 15 | 1.04% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |

