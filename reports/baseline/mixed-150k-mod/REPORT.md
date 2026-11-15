# Baseline run: `mixed-150k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 112341 -> 112280) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 88.256 / 140.572 / 263.107 / 367.585 |
| MSPT mean, ms | 102.479 |
| TPS estimate (1000/p50) | 11.3 |
| heap used / max, MB | 2050.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8734` |
| 2 | 9.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7923` |
| 3 | 6.02% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13654` |
| 4 | 5.93% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 4.94% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 4.51% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.59% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5394` |
| 9 | 1.91% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13001` |
| 10 | 1.35% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 11 | 1.30% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 12 | 1.10% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.04% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 0.86% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 15 | 0.84% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13645` |

