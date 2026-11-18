# Baseline run: `mixed-150k-mod`

Server: `26.3-255-c22378f (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111846 -> 111265) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 75.438 / 139.591 / 281.771 / 527.074 |
| MSPT mean, ms | 93.679 |
| TPS estimate (1000/p50) | 13.3 |
| heap used / max, MB | 2192.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9870` |
| 2 | 7.24% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.74% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14975` |
| 4 | 4.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8727` |
| 5 | 2.79% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.56% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14210` |
| 7 | 1.74% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 984` |
| 8 | 1.49% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 9 | 1.32% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5711` |
| 10 | 1.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 15031` |
| 11 | 1.22% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 1.19% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 13 | 1.06% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 14 | 1.04% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 15 | 0.96% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14400` |

