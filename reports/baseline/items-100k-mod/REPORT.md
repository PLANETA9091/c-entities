# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63041 -> 63041) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 28.596 / 72.003 / 152.485 / 204.924 |
| MSPT mean, ms | 38.914 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1220.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 2 | 11.04% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5033` |
| 3 | 4.63% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.64% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 3.56% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 6 | 3.26% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 3.09% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 8 | 2.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3607` |
| 9 | 2.67% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3734` |
| 10 | 1.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.84% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 13 | 1.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 14 | 1.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5097` |
| 15 | 1.07% | `net.minecraft.world.entity.Entity.getType() line: 633` |

