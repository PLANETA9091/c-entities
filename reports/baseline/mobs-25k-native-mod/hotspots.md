# JVM hotspots (JFR ExecutionSample)

Samples: 2697

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.64% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 3.63% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16449` |
| 3 | 2.52% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15240` |
| 4 | 2.45% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 5 | 1.93% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.78% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.74% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 8 | 1.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.45% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 11 | 1.45% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 12 | 1.37% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10756` |
| 14 | 1.04% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 15 | 1.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 16 | 0.93% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 17 | 0.93% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 18 | 0.89% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 19 | 0.85% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 20 | 0.67% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16468` |
| 21 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 22 | 0.59% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 23 | 0.59% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 24 | 0.59% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2704` |
| 25 | 0.59% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 26 | 0.56% | `java.util.HashMap.getNode(Object) line: 586` |
| 27 | 0.52% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 28 | 0.48% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 29 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 30 | 0.44% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 31 | 0.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 32 | 0.41% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 33 | 0.41% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 34 | 0.41% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 35 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 36 | 0.37% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 416` |
| 37 | 0.33% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 38 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 39 | 0.33% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 40 | 0.33% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 41 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 42 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 43 | 0.30% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 44 | 0.30% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 45 | 0.26% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 46 | 0.26% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 47 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 934` |
| 48 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 49 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 50 | 0.22% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 51 | 0.22% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 52 | 0.22% | `java.util.ArrayList.grow(int) line: 238` |
| 53 | 0.22% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 54 | 0.22% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 55 | 0.22% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 56 | 0.19% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 57 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 58 | 0.19% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 59 | 0.19% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 60 | 0.19% | `org.objectweb.asm.commons.Remapper.mapDesc(String) line: 104` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.30% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13879` |
| 2 | 4.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16727` |
| 3 | 4.41% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13812` |
| 4 | 2.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 5 | 2.78% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.67% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 7 | 1.93% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.74% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 9 | 1.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 11 | 1.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16453` |
| 12 | 1.37% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15248` |
| 13 | 1.26% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 14 | 1.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8508` |
| 15 | 1.00% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 16 | 0.93% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2207` |
| 17 | 0.89% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16456` |
| 18 | 0.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10851` |
| 19 | 0.82% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13832` |
| 20 | 0.78% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 21 | 0.67% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.67% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16458` |
| 23 | 0.56% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 25 | 0.52% | `java.util.HashMap.containsKey(Object) line: 603` |
| 26 | 0.52% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 27 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.has(Entity) line: 473` |
| 28 | 0.48% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 29 | 0.41% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 30 | 0.41% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

