# Baseline run: `mixed-150k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100606 -> 100606) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 237.590 / 317.266 / 411.407 / 477.280 |
| MSPT mean, ms | 248.635 |
| TPS estimate (1000/p50) | 4.2 |
| heap used / max, MB | 2590.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 2.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 3 | 2.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 4 | 2.69% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7525` |
| 6 | 2.48% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 2.45% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7666` |
| 8 | 2.08% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4440` |
| 9 | 2.00% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7472` |
| 10 | 1.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4585` |
| 11 | 1.61% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.49% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.46% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 15 | 1.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |

