# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62913 -> 62913) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 25.188 / 69.004 / 124.067 / 190.868 |
| MSPT mean, ms | 34.620 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1517.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 8.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 3 | 6.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 5.64% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 5 | 4.83% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 6 | 4.60% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.25% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2764` |
| 8 | 3.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 9 | 2.71% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 2.07% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 11 | 1.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4121` |
| 12 | 1.84% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 13 | 1.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 14 | 1.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 1.04% | `net.minecraft.world.entity.Entity.getType() line: 633` |

