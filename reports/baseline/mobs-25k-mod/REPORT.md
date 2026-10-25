# Baseline run: `mobs-25k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 19957 -> 19938) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 205.631 / 245.885 / 278.827 / 313.322 |
| MSPT mean, ms | 214.014 |
| TPS estimate (1000/p50) | 4.9 |
| heap used / max, MB | 1550.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.04% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 1.70% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 1.61% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 4423` |
| 5 | 1.38% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 4394` |
| 6 | 1.37% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1495` |
| 7 | 1.24% | `java.util.HashMap.getNode(Object) line: 586` |
| 8 | 1.05% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 9 | 1.03% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 10 | 0.99% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 0.98% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3953` |
| 12 | 0.96% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 0.94% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1405` |
| 14 | 0.94% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |

