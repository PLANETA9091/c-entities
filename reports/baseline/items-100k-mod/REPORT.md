# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62909 -> 62909) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 28.982 / 91.124 / 163.074 / 233.576 |
| MSPT mean, ms | 39.825 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1500.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 10.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5473` |
| 3 | 6.38% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4054` |
| 5 | 2.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 6 | 1.52% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3927` |
| 8 | 1.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5478` |
| 9 | 1.24% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 10 | 1.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5549` |
| 11 | 1.16% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.11% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9470` |
| 13 | 1.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.09% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9473` |
| 15 | 1.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6241` |

