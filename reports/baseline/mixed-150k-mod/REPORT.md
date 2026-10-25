# Baseline run: `mixed-150k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100370 -> 100370) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 350.654 / 426.657 / 474.858 / 499.907 |
| MSPT mean, ms | 362.704 |
| TPS estimate (1000/p50) | 2.9 |
| heap used / max, MB | 2016.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.03% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.24% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 4065` |
| 3 | 3.81% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3148` |
| 4 | 3.81% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 3.40% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1493` |
| 6 | 2.93% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4034` |
| 7 | 2.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.98% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3284` |
| 12 | 1.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.24% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.21% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3624` |
| 15 | 1.18% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |

