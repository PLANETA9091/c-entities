# Baseline run: `mixed-150k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111726 -> 111313) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 57.822 / 99.539 / 230.496 / 299.683 |
| MSPT mean, ms | 72.345 |
| TPS estimate (1000/p50) | 17.3 |
| heap used / max, MB | 1968.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10555` |
| 2 | 6.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15846` |
| 3 | 6.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 5.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9255` |
| 5 | 3.60% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.58% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14966` |
| 7 | 1.70% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 8 | 1.63% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 9 | 1.45% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 10 | 1.41% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5938` |
| 11 | 1.28% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 12 | 1.09% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 0.92% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15189` |
| 14 | 0.90% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 15 | 0.89% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |

