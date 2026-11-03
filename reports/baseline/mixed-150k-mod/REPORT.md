# Baseline run: `mixed-150k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109698 -> 109698) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 250.617 / 361.572 / 525.766 / 624.177 |
| MSPT mean, ms | 277.838 |
| TPS estimate (1000/p50) | 4.0 |
| heap used / max, MB | 1922.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 2 | 3.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.32% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 2.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9759` |
| 5 | 2.03% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10361` |
| 6 | 1.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.82% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 9 | 1.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.34% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.15% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.11% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 1.05% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |

