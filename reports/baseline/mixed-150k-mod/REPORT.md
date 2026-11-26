# Baseline run: `mixed-150k-mod`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111352 -> 111297) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 62.212 / 104.943 / 255.054 / 407.652 |
| MSPT mean, ms | 76.857 |
| TPS estimate (1000/p50) | 16.1 |
| heap used / max, MB | 1914.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10831` |
| 2 | 6.00% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16558` |
| 3 | 5.61% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9530` |
| 6 | 2.77% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15288` |
| 7 | 2.34% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13912` |
| 8 | 1.42% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 9 | 1.38% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 10 | 1.17% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6772` |
| 11 | 1.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9276` |
| 12 | 1.09% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16577` |
| 13 | 0.95% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 0.93% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 0.84% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

