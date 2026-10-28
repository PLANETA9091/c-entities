# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62977 -> 62977) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 38.707 / 108.368 / 187.535 / 209.044 |
| MSPT mean, ms | 51.799 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1584.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 2 | 8.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4073` |
| 3 | 6.63% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.20% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2769` |
| 5 | 5.02% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 4.43% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.96% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 8 | 2.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 1.98% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 1.81% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2671` |
| 11 | 1.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 12 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4079` |
| 13 | 1.10% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 1.02% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 15 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

