# Baseline run: `mixed-150k-mod`

Server: `26.3-259-c7097a9 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111960 -> 111434) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 61.826 / 109.069 / 255.882 / 430.417 |
| MSPT mean, ms | 78.272 |
| TPS estimate (1000/p50) | 16.2 |
| heap used / max, MB | 2246.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10701` |
| 2 | 6.61% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15992` |
| 3 | 5.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9400` |
| 5 | 3.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.67% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15112` |
| 7 | 1.82% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 8 | 1.44% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 9 | 1.41% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 10 | 1.35% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15335` |
| 11 | 1.28% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 12 | 1.26% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6706` |
| 13 | 1.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9161` |
| 14 | 1.13% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 15 | 1.11% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |

