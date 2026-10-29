# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62879 -> 62879) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.964 / 85.342 / 141.623 / 252.213 |
| MSPT mean, ms | 40.541 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1177.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7836` |
| 2 | 7.90% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4178` |
| 3 | 6.03% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.75% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 5 | 5.67% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7783` |
| 6 | 4.62% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.78% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2873` |
| 8 | 2.67% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2765` |
| 10 | 2.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.81% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4896` |
| 13 | 1.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4184` |
| 14 | 0.88% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 2826` |
| 15 | 0.86% | `net.minecraft.world.entity.Entity.getType() line: 633` |

