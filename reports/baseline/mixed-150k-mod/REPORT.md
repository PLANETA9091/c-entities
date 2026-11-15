# Baseline run: `mixed-150k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 112227 -> 112130) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 93.162 / 146.234 / 303.060 / 354.622 |
| MSPT mean, ms | 104.911 |
| TPS estimate (1000/p50) | 10.7 |
| heap used / max, MB | 2130.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 6.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7884` |
| 3 | 6.73% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 5.78% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13538` |
| 5 | 5.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8646` |
| 6 | 3.37% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.58% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.51% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.83% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.73% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5054` |
| 11 | 1.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5322` |
| 12 | 1.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5326` |
| 13 | 1.22% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 1.09% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2504` |
| 15 | 1.08% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |

