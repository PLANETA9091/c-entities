# Baseline run: `mixed-150k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111715 -> 111248) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 81.494 / 144.645 / 351.311 / 488.734 |
| MSPT mean, ms | 98.009 |
| TPS estimate (1000/p50) | 12.3 |
| heap used / max, MB | 2261.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9568` |
| 2 | 6.99% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14614` |
| 3 | 6.87% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8725` |
| 5 | 2.65% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13849` |
| 6 | 2.51% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.89% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 984` |
| 8 | 1.44% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 9 | 1.35% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 10 | 1.31% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 11 | 1.24% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5709` |
| 12 | 1.03% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14670` |
| 13 | 0.98% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 14 | 0.97% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 0.97% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14039` |

