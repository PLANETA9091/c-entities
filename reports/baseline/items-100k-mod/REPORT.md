# Baseline run: `items-100k-mod`

Server: `26.3-235-bb10cf0 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62943 -> 62943) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 36.857 / 89.162 / 160.900 / 209.614 |
| MSPT mean, ms | 47.768 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1434.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.69% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5033` |
| 2 | 10.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 3 | 4.94% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3734` |
| 4 | 4.65% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.14% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 6 | 2.86% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 7 | 2.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3607` |
| 8 | 2.40% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 2.08% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 10 | 1.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.56% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5097` |
| 13 | 1.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 14 | 1.28% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 1.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |

