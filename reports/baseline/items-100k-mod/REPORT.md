# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62964 -> 62964) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 24.427 / 68.955 / 126.881 / 171.639 |
| MSPT mean, ms | 33.643 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1148.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7721` |
| 2 | 8.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4072` |
| 3 | 5.88% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 5.05% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2768` |
| 5 | 4.65% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7668` |
| 6 | 4.26% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.90% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 2.88% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 9 | 2.70% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2670` |
| 10 | 2.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 2.05% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2721` |
| 12 | 1.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4781` |
| 13 | 1.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 1.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4078` |
| 15 | 1.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4783` |

