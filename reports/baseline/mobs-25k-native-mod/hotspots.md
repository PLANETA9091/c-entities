# JVM hotspots (JFR ExecutionSample)

Samples: 2733

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.63% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 3.77% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15438` |
| 3 | 2.78% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14572` |
| 4 | 2.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10216` |
| 5 | 2.52% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.20% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 7 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.54% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 10 | 1.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.10% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 1.02% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 13 | 0.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 0.88% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 0.80% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 16 | 0.66% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15459` |
| 17 | 0.66% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2626` |
| 18 | 0.62% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 19 | 0.62% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 20 | 0.59% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 21 | 0.59% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 22 | 0.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 23 | 0.55% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 24 | 0.55% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 25 | 0.51% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 26 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 27 | 0.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 28 | 0.48% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 29 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 30 | 0.40% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 31 | 0.40% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 32 | 0.40% | `java.util.HashMap.getNode(Object) line: 586` |
| 33 | 0.37% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 34 | 0.37% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 35 | 0.37% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 36 | 0.37% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 37 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 38 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 39 | 0.37% | `java.util.ArrayList$Itr.next() line: 1058` |
| 40 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 41 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 42 | 0.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15718` |
| 43 | 0.33% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13197` |
| 44 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 446` |
| 45 | 0.29% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5173` |
| 46 | 0.29% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 47 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 48 | 0.26% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 49 | 0.26% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 50 | 0.26% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.26% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 52 | 0.26% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5200` |
| 53 | 0.26% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 54 | 0.26% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5188` |
| 55 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 56 | 0.22% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 57 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 58 | 0.22% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 59 | 0.22% | `java.util.HashMap.getNode(Object) line: 579` |
| 60 | 0.22% | `java.lang.Math.unsignedMultiplyHigh(long, long) line: 1433` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.34% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13214` |
| 2 | 4.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15718` |
| 3 | 3.22% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 4 | 2.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.78% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13147` |
| 6 | 2.67% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 7 | 2.60% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 2.52% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.72% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 10 | 1.54% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 11 | 1.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 12 | 1.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10236` |
| 13 | 1.17% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 14 | 1.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8109` |
| 15 | 1.10% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 16 | 1.06% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14580` |
| 17 | 0.95% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 18 | 0.84% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15447` |
| 19 | 0.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 20 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 0.62% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 22 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.55% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.55% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 25 | 0.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15444` |
| 26 | 0.51% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.has(Entity) line: 473` |
| 28 | 0.48% | `java.util.HashMap.containsKey(Object) line: 603` |
| 29 | 0.48% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 30 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |

GC pauses: 0, total 0 ms

