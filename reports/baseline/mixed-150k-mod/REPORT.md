# Baseline run: `mixed-150k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111743 -> 111324) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 58.923 / 103.753 / 233.214 / 398.216 |
| MSPT mean, ms | 73.241 |
| TPS estimate (1000/p50) | 17.0 |
| heap used / max, MB | 1950.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10331` |
| 2 | 6.67% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.79% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15622` |
| 4 | 3.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9082` |
| 5 | 3.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.50% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14742` |
| 7 | 2.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8843` |
| 8 | 1.77% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15643` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5924` |
| 10 | 1.56% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 11 | 1.31% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 1.30% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 13 | 1.12% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 1.10% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 15 | 0.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10375` |

