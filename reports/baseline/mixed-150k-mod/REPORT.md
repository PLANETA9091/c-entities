# Baseline run: `mixed-150k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109725 -> 109725) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 317.581 / 415.024 / 540.623 / 607.036 |
| MSPT mean, ms | 333.471 |
| TPS estimate (1000/p50) | 3.1 |
| heap used / max, MB | 2203.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.87% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.40% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7819` |
| 4 | 3.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4879` |
| 5 | 2.95% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.23% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7960` |
| 7 | 1.92% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7766` |
| 8 | 1.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.72% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4734` |
| 10 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4161` |
| 12 | 1.47% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.14% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 14 | 1.13% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 15 | 1.12% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |

