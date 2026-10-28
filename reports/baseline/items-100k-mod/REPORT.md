# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62942 -> 62942) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 31.278 / 87.306 / 156.220 / 197.992 |
| MSPT mean, ms | 42.351 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1432.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 8.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 3 | 7.37% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.79% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 5 | 4.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 4.24% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.66% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 8 | 2.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 2.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 10 | 1.75% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 11 | 1.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 12 | 1.21% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.13% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 14 | 1.05% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 0.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4121` |

