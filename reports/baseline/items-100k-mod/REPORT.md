# Baseline run: `items-100k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54734 -> 54734) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 13.895 / 16.556 / 82.089 / 114.845 |
| MSPT mean, ms | 15.611 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1245.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.68% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 11.92% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4006` |
| 3 | 5.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |
| 4 | 4.26% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6607` |
| 5 | 2.94% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 6 | 2.72% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.59% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 8 | 2.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2080` |
| 9 | 1.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2034` |
| 10 | 1.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.38% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 12 | 1.27% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.24% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 14 | 1.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 1.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

