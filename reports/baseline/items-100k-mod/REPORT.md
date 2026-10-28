# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62913 -> 62913) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 33.244 / 92.574 / 147.435 / 212.638 |
| MSPT mean, ms | 45.612 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1291.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 10.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 3 | 6.02% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.79% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.15% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 4.03% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 7 | 3.64% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 8 | 1.94% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.08% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 12 | 0.90% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 13 | 0.83% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 14 | 0.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 15 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

