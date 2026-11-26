# JVM hotspots (JFR ExecutionSample)

Samples: 3178

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.81% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16558` |
| 2 | 4.56% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13912` |
| 3 | 2.20% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 1.89% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 5 | 1.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10758` |
| 6 | 1.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 7 | 1.54% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15288` |
| 8 | 1.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.26% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 10 | 0.88% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 11 | 0.88% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 0.79% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 13 | 0.76% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.72% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 15 | 0.69% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 16 | 0.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 17 | 0.66% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16836` |
| 18 | 0.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 19 | 0.53% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 20 | 0.53% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2706` |
| 21 | 0.47% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 22 | 0.44% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 23 | 0.44% | `java.util.HashMap.getNode(Object) line: 586` |
| 24 | 0.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 25 | 0.38% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 26 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 27 | 0.38% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 29 | 0.35% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 30 | 0.35% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 31 | 0.35% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 32 | 0.35% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 33 | 0.35% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.task.ChunkLoadTask.<init>(ChunkTaskScheduler, ServerLevel, int, int, NewChunkHolder, Priority) line: 49` |
| 35 | 0.35% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.35% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 37 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16577` |
| 38 | 0.31% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 39 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 934` |
| 40 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 41 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 42 | 0.28% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 43 | 0.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 44 | 0.28% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 45 | 0.28% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 12549` |
| 46 | 0.25% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 47 | 0.25% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 48 | 0.25% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 49 | 0.25% | `java.lang.Class.cast(Object) line: 3491` |
| 50 | 0.25% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 51 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.25% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13900` |
| 53 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 54 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 55 | 0.22% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 56 | 0.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10852` |
| 57 | 0.22% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 12557` |
| 58 | 0.22% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 59 | 0.22% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 60 | 0.19% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8510` |
| 2 | 5.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16836` |
| 3 | 3.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 4 | 2.45% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13823` |
| 5 | 2.27% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.20% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.83% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 1.42% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 10 | 1.23% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 11 | 1.23% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 12 | 1.07% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 13 | 0.88% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15296` |
| 14 | 0.79% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16562` |
| 15 | 0.76% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 16 | 0.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10853` |
| 17 | 0.66% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 18 | 0.66% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 19 | 0.66% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13843` |
| 20 | 0.57% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.50% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.47% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 23 | 0.47% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 24 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 25 | 0.41% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 26 | 0.41% | `java.util.HashMap.containsKey(Object) line: 603` |
| 27 | 0.38% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 28 | 0.38% | `net.minecraft.world.level.levelgen.synth.NoiseStack$SmearedPerlin.addToVolume(DensityBuffer, DensityVolume, double, double, float) line: 153` |
| 29 | 0.35% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.isJoinNonEmpty(VoxelShape, VoxelShape, BooleanOp) line: 1320` |
| 30 | 0.35% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1160` |

GC pauses: 0, total 0 ms

