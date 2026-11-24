# Baseline run: `mixed-150k-mod`

Server: `26.3-262-827eeeb (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111715 -> 111285) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 55.022 / 84.465 / 226.105 / 332.437 |
| MSPT mean, ms | 66.662 |
| TPS estimate (1000/p50) | 18.2 |
| heap used / max, MB | 2008.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10798` |
| 2 | 7.41% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 6.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16241` |
| 4 | 3.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9497` |
| 5 | 3.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.62% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15209` |
| 7 | 1.77% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16262` |
| 8 | 1.52% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 9 | 1.48% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 10 | 1.43% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 11 | 1.19% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6025` |
| 12 | 1.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9243` |
| 13 | 1.09% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 14 | 1.07% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 1.05% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

