# Baseline run: `mixed-150k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111789 -> 111338) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 59.116 / 102.461 / 220.538 / 316.929 |
| MSPT mean, ms | 72.357 |
| TPS estimate (1000/p50) | 16.9 |
| heap used / max, MB | 1785.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10231` |
| 2 | 7.32% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 6.64% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15438` |
| 4 | 4.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9023` |
| 5 | 3.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.35% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14572` |
| 7 | 2.21% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 8 | 1.85% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 9 | 1.67% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 10 | 1.60% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5904` |
| 11 | 1.16% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.08% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1024` |
| 13 | 1.05% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 0.97% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 15 | 0.97% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |

