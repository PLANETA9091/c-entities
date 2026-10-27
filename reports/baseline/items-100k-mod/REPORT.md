# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63041 -> 63041) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 28.259 / 76.386 / 137.903 / 186.039 |
| MSPT mean, ms | 38.063 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1335.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7554` |
| 2 | 7.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3905` |
| 3 | 4.69% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.58% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.70% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7501` |
| 7 | 3.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2503` |
| 8 | 3.15% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2601` |
| 9 | 2.55% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 2.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.65% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.37% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2554` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3911` |
| 15 | 0.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4614` |

