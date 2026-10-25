# Baseline run: `items-100k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54823 -> 54823) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 18.845 / 23.405 / 98.247 / 111.832 |
| MSPT mean, ms | 21.649 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1150.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.42% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 7.86% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3671` |
| 3 | 6.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3817` |
| 4 | 6.05% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3681` |
| 5 | 5.08% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5439` |
| 6 | 3.69% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 7 | 1.85% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 8 | 1.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3116` |
| 9 | 1.53% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 1.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 11 | 1.29% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 12 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 1.04% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 14 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.93% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |

