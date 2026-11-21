# Baseline run: `mixed-150k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111675 -> 111257) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 56.346 / 93.003 / 223.812 / 348.287 |
| MSPT mean, ms | 69.544 |
| TPS estimate (1000/p50) | 17.7 |
| heap used / max, MB | 1885.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10309` |
| 2 | 7.22% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15535` |
| 3 | 6.42% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 5.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9082` |
| 5 | 3.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.13% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14655` |
| 7 | 2.07% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 8 | 1.67% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 9 | 1.43% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5924` |
| 10 | 1.38% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 11 | 1.14% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 12 | 1.10% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 13 | 1.10% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1024` |
| 15 | 1.06% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14878` |

