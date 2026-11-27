# Baseline run: `mixed-150k-mod`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111850 -> 111310) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 62.860 / 114.857 / 270.378 / 345.633 |
| MSPT mean, ms | 79.820 |
| TPS estimate (1000/p50) | 15.9 |
| heap used / max, MB | 2043.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10834` |
| 2 | 6.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16571` |
| 3 | 5.47% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9533` |
| 5 | 3.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.68% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15291` |
| 7 | 2.48% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13915` |
| 8 | 1.70% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 9 | 1.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9279` |
| 10 | 1.29% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 11 | 1.18% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6775` |
| 12 | 1.15% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.12% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 14 | 0.96% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6034` |
| 15 | 0.94% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16590` |

