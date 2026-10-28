# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62927 -> 62927) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 25.823 / 62.066 / 131.106 / 174.885 |
| MSPT mean, ms | 35.029 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1292.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 8.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 3 | 6.28% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 5.85% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 5 | 4.89% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 6 | 4.30% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 7 | 4.26% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.70% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 10 | 1.81% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2764` |
| 11 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 13 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4121` |
| 14 | 1.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 15 | 1.13% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |

