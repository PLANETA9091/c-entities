# JVM hotspots (JFR ExecutionSample)

Samples: 2452

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.96% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12887` |
| 2 | 3.92% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 4 | 2.57% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 2.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.00% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 7 | 1.88% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 8 | 1.88% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12906` |
| 9 | 1.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8219` |
| 11 | 1.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.67% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 13 | 1.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13129` |
| 14 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.22% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 16 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 0.73% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 18 | 0.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 19 | 0.65% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.61% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 21 | 0.61% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 22 | 0.57% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 23 | 0.53% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 24 | 0.53% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 25 | 0.49% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 26 | 0.49% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1916` |
| 27 | 0.45% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 28 | 0.45% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 29 | 0.45% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 30 | 0.45% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 31 | 0.41% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 32 | 0.41% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 33 | 0.37% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 34 | 0.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 35 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 36 | 0.33% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 37 | 0.33% | `java.util.HashMap.getNode(Object) line: 586` |
| 38 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 39 | 0.33% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5188` |
| 40 | 0.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13025` |
| 41 | 0.33% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 42 | 0.33% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 43 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 44 | 0.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 45 | 0.29% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 46 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 47 | 0.24% | `java.util.Objects.equals(Object, Object) line: 62` |
| 48 | 0.24% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 49 | 0.24% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 50 | 0.24% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1910` |
| 51 | 0.24% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 52 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 883` |
| 53 | 0.20% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 54 | 0.20% | `net.minecraft.world.level.levelgen.material.rule.ConditionRule.lambda$compile$0(ConditionEvaluator, RuleEvaluator, int, int, int) line: 27` |
| 55 | 0.20% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 56 | 0.20% | `java.lang.Math.unsignedMultiplyHigh(long, long) line: 1433` |
| 57 | 0.20% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 765` |
| 59 | 0.20% | `it.unimi.dsi.fastutil.ints.Int2IntOpenHashMap.find(int) line: 238` |
| 60 | 0.20% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13129` |
| 2 | 3.83% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.63% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11128` |
| 4 | 3.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 5 | 2.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 6 | 2.04% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.00% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 8 | 1.96% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 9 | 1.92% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11081` |
| 10 | 1.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.67% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11061` |
| 12 | 1.63% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 13 | 1.63% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11089` |
| 14 | 1.51% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 15 | 1.18% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 16 | 0.94% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 17 | 0.94% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 18 | 0.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6822` |
| 19 | 0.73% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.73% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 21 | 0.65% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5735` |
| 22 | 0.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 23 | 0.57% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8245` |
| 25 | 0.49% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |
| 26 | 0.45% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.45% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.45% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 29 | 0.41% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 30 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |

GC pauses: 0, total 0 ms

