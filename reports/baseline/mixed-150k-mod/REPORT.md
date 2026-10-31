# Baseline run: `mixed-150k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109624 -> 109624) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 246.067 / 372.055 / 591.638 / 694.530 |
| MSPT mean, ms | 280.660 |
| TPS estimate (1000/p50) | 4.1 |
| heap used / max, MB | 1947.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.49% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.82% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 4 | 3.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 5 | 3.35% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 3.24% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.18% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 2.07% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8929` |
| 9 | 2.02% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5679` |
| 10 | 1.91% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 11 | 1.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5033` |
| 13 | 1.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.24% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 15 | 1.24% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |

