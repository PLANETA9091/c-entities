# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62903 -> 62903) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 22.843 / 58.049 / 113.394 / 170.650 |
| MSPT mean, ms | 31.246 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1243.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7656` |
| 2 | 6.87% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 3 | 6.48% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7603` |
| 4 | 6.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4007` |
| 5 | 4.84% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2703` |
| 6 | 3.29% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2656` |
| 7 | 3.27% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 3.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2605` |
| 9 | 3.01% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 2.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4718` |
| 11 | 1.98% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 12 | 1.95% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4013` |
| 14 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 1.18% | `net.minecraft.world.entity.Entity.getType() line: 633` |

