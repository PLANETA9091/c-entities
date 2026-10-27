# Baseline run: `mixed-150k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100579 -> 100579) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 255.598 / 345.656 / 463.656 / 570.743 |
| MSPT mean, ms | 271.517 |
| TPS estimate (1000/p50) | 3.9 |
| heap used / max, MB | 1998.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.26% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7603` |
| 3 | 3.03% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 4 | 2.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 8 | 2.03% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 2.02% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4377` |
| 10 | 1.89% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 11 | 1.80% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 12 | 1.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1715` |
| 14 | 1.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.08% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |

