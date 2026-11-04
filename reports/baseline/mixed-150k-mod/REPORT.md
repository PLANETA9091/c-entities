# Baseline run: `mixed-150k-mod`

Server: `26.3-241-5050b0f (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109678 -> 109678) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 407.962 / 501.696 / 692.876 / 838.171 |
| MSPT mean, ms | 432.572 |
| TPS estimate (1000/p50) | 2.5 |
| heap used / max, MB | 1969.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.86% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 2 | 10.24% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.18% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 4 | 2.94% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10280` |
| 5 | 2.42% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 7 | 1.51% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 8 | 1.35% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 9 | 1.27% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2410` |
| 10 | 1.22% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10916` |
| 11 | 1.17% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.16% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.11% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2399` |
| 14 | 1.11% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.05% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |

