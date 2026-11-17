# Baseline run: `mixed-150k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111852 -> 111330) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 71.642 / 131.070 / 280.965 / 400.090 |
| MSPT mean, ms | 88.054 |
| TPS estimate (1000/p50) | 14.0 |
| heap used / max, MB | 2294.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.97% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 10.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9414` |
| 3 | 6.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14420` |
| 4 | 4.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8571` |
| 5 | 3.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.40% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7608` |
| 7 | 1.75% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13695` |
| 8 | 1.52% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 9 | 1.28% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 10 | 1.16% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 11 | 1.06% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.03% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 13 | 0.96% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 14 | 0.87% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 0.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5901` |

