# Baseline run: `mixed-150k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109749 -> 109749) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 235.838 / 342.095 / 469.562 / 564.732 |
| MSPT mean, ms | 261.665 |
| TPS estimate (1000/p50) | 4.2 |
| heap used / max, MB | 1871.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.98% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5987` |
| 3 | 3.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8964` |
| 5 | 3.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.09% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9105` |
| 7 | 2.89% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 2.20% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8911` |
| 9 | 2.05% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 2.03% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5833` |
| 11 | 1.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5251` |
| 13 | 1.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.16% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |

