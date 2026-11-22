# JVM hotspots (JFR ExecutionSample)

Samples: 2549

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.34% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 4.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15846` |
| 3 | 2.47% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14966` |
| 4 | 2.24% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 5 | 2.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 1.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.53% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 9 | 1.37% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.10% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 12 | 1.06% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 13 | 0.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10482` |
| 14 | 0.90% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15867` |
| 15 | 0.86% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 16 | 0.86% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 17 | 0.75% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 18 | 0.71% | `java.util.HashMap.getNode(Object) line: 586` |
| 19 | 0.71% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 20 | 0.67% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 21 | 0.67% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 22 | 0.67% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 23 | 0.63% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 24 | 0.59% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 25 | 0.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 26 | 0.55% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2641` |
| 27 | 0.51% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 28 | 0.51% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 29 | 0.51% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 30 | 0.47% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 31 | 0.47% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 32 | 0.43% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 33 | 0.43% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 34 | 0.39% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 35 | 0.39% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2774` |
| 36 | 0.35% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 37 | 0.35% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 38 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 39 | 0.35% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 40 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 41 | 0.35% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 42 | 0.31% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 43 | 0.27% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 44 | 0.27% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2621` |
| 45 | 0.27% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 46 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 47 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 48 | 0.24% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 49 | 0.24% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 50 | 0.24% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 51 | 0.24% | `com.mojang.serialization.Encoder$1.encode(Object, DynamicOps, Object) line: 25` |
| 52 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 446` |
| 53 | 0.24% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 54 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 55 | 0.20% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 56 | 0.20% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.joinUnoptimized(VoxelShape, VoxelShape, BooleanOp) line: 1261` |
| 57 | 0.20% | `net.minecraft.world.level.biome.Climate$RTree.buildParameterSpace(List) line: 455` |
| 58 | 0.20% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 59 | 0.20% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 60 | 0.20% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16126` |
| 2 | 5.14% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13605` |
| 3 | 3.81% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13538` |
| 4 | 2.90% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 6 | 2.79% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 7 | 2.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 9 | 1.53% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 10 | 1.26% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.26% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 12 | 1.22% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 13 | 1.10% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 14 | 0.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8322` |
| 15 | 0.90% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 16 | 0.86% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14974` |
| 17 | 0.78% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13558` |
| 18 | 0.75% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 0.75% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15855` |
| 20 | 0.67% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 21 | 0.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10577` |
| 22 | 0.63% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 23 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.59% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.55% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.55% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.55% | `java.util.HashMap.containsKey(Object) line: 603` |
| 28 | 0.47% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2647` |
| 29 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15852` |
| 30 | 0.43% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |

GC pauses: 0, total 0 ms

