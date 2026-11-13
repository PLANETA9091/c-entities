# JVM hotspots (JFR ExecutionSample)

Samples: 3616

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.72% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11848` |
| 2 | 4.76% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.71% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.65% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 1.99% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.60% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 1.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7346` |
| 8 | 1.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.27% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 10 | 1.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.05% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 12 | 0.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12072` |
| 13 | 0.77% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.75% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 0.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 16 | 0.72% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11867` |
| 17 | 0.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 18 | 0.55% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 19 | 0.53% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 20 | 0.53% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 21 | 0.50% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 22 | 0.47% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 23 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 24 | 0.39% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 25 | 0.39% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 26 | 0.39% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 27 | 0.39% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1891` |
| 28 | 0.39% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 29 | 0.36% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 30 | 0.36% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 31 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 32 | 0.33% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 33 | 0.33% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 34 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 35 | 0.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11977` |
| 36 | 0.33% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 37 | 0.30% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 38 | 0.30% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 39 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 40 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11885` |
| 41 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 42 | 0.28% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 8942` |
| 43 | 0.28% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 44 | 0.28% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 45 | 0.28% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 46 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 47 | 0.25% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 48 | 0.25% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 49 | 0.25% | `java.util.ArrayList$Itr.next() line: 1058` |
| 50 | 0.25% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 51 | 0.25% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 52 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 53 | 0.22% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 54 | 0.22% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 55 | 0.22% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 56 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 57 | 0.22% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 58 | 0.22% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 59 | 0.22% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 60 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12072` |
| 2 | 4.65% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10255` |
| 3 | 4.26% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10188` |
| 4 | 2.96% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 5 | 2.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 6 | 1.99% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.44% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.33% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5122` |
| 11 | 1.27% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 12 | 1.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 13 | 1.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6193` |
| 14 | 1.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7372` |
| 15 | 0.91% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 16 | 0.75% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 17 | 0.72% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 18 | 0.66% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 19 | 0.61% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.58% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.53% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 22 | 0.47% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 23 | 0.47% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 24 | 0.47% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.39% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 26 | 0.36% | `java.util.zip.ZipFile.getEntry(String) line: 294` |
| 27 | 0.33% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.33% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 29 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |

GC pauses: 0, total 0 ms

