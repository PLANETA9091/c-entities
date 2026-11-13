# Baseline run: `mobs-25k-native-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25049 -> 25041) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 40.087 / 73.707 / 194.117 / 212.027 |
| MSPT mean, ms | 50.092 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1532.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.78% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11919` |
| 2 | 5.11% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.58% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 3.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7416` |
| 5 | 3.01% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 2.00% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 1.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.59% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 1.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.12% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.08% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 0.95% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

