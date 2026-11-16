# Baseline run: `mixed-150k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 112325 -> 112279) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 90.057 / 162.281 / 319.647 / 425.234 |
| MSPT mean, ms | 108.013 |
| TPS estimate (1000/p50) | 11.1 |
| heap used / max, MB | 2118.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8945` |
| 2 | 9.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8102` |
| 3 | 5.74% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13865` |
| 4 | 5.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 4.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.90% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.23% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13212` |
| 9 | 1.95% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5566` |
| 10 | 1.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.35% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 12 | 1.25% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.15% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 0.97% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 15 | 0.93% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |

