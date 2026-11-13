# Baseline run: `mobs-25k-native-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25062 -> 25052) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 46.818 / 72.783 / 217.276 / 385.902 |
| MSPT mean, ms | 56.848 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1546.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.65% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11849` |
| 2 | 4.41% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.33% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 1.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 7 | 1.57% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7347` |
| 9 | 1.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.28% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 11 | 1.28% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 12 | 0.81% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 0.75% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 14 | 0.70% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1377` |
| 15 | 0.67% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |

