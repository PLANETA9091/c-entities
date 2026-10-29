# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62896 -> 62896) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 24.370 / 60.248 / 132.308 / 229.590 |
| MSPT mean, ms | 33.456 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1189.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7836` |
| 2 | 8.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4178` |
| 3 | 6.89% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 5.04% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7783` |
| 5 | 4.83% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2873` |
| 6 | 4.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.24% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 3.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2765` |
| 9 | 2.63% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 10 | 2.27% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 11 | 2.13% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 2826` |
| 12 | 1.64% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4898` |
| 13 | 1.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4184` |
| 14 | 1.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 1.08% | `net.minecraft.world.entity.Entity.getType() line: 633` |

