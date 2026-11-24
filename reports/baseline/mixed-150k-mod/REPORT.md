# Baseline run: `mixed-150k-mod`

Server: `26.3-262-827eeeb (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111798 -> 111277) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 56.027 / 99.069 / 226.701 / 327.543 |
| MSPT mean, ms | 70.989 |
| TPS estimate (1000/p50) | 17.8 |
| heap used / max, MB | 1719.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10799` |
| 2 | 6.04% | `dev.planeta9091.centities.CeHooks.sitStepsEmpty(Object) line: 15343` |
| 3 | 4.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.07% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.80% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15210` |
| 6 | 2.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9498` |
| 7 | 1.54% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 8 | 1.50% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 9 | 1.49% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15459` |
| 10 | 1.24% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 11 | 1.17% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 12 | 1.02% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9244` |
| 14 | 1.01% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6767` |
| 15 | 1.00% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16310` |

