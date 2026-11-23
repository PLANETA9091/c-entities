# JVM hotspots (JFR ExecutionSample)

Samples: 2797

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.90% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 4.00% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15992` |
| 3 | 2.25% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15112` |
| 4 | 1.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 1.68% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 1.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 1.54% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 8 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10628` |
| 11 | 1.11% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 12 | 1.07% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16013` |
| 13 | 0.89% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 0.82% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 0.79% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 16 | 0.72% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 17 | 0.72% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 18 | 0.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 19 | 0.68% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 20 | 0.68% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 21 | 0.64% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 22 | 0.64% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 23 | 0.54% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 24 | 0.50% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 25 | 0.50% | `java.util.HashMap.getNode(Object) line: 586` |
| 26 | 0.50% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 27 | 0.50% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2641` |
| 28 | 0.46% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 29 | 0.46% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 30 | 0.43% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 31 | 0.39% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 32 | 0.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 33 | 0.39% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 34 | 0.39% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 35 | 0.36% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 36 | 0.32% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 37 | 0.32% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 38 | 0.32% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 39 | 0.32% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 40 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 41 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 42 | 0.32% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 43 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 44 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 45 | 0.29% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 46 | 0.29% | `java.util.ArrayList.isEmpty() line: 263` |
| 47 | 0.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16272` |
| 48 | 0.29% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5173` |
| 49 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 50 | 0.25% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 51 | 0.25% | `java.util.ArrayList$Itr.next() line: 1058` |
| 52 | 0.25% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 53 | 0.25% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 54 | 0.25% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15335` |
| 55 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 56 | 0.21% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 30` |
| 57 | 0.21% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 58 | 0.21% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 59 | 0.21% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 60 | 0.21% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.65% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13751` |
| 2 | 5.65% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16272` |
| 3 | 3.29% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13684` |
| 4 | 2.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 5 | 2.68% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 6 | 2.22% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.68% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 9 | 1.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 10 | 1.47% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8397` |
| 12 | 1.00% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 13 | 0.93% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 14 | 0.89% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10723` |
| 15 | 0.86% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 0.72% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 17 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 18 | 0.72% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16001` |
| 19 | 0.68% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15120` |
| 20 | 0.68% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15998` |
| 21 | 0.64% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.64% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 23 | 0.57% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.54% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16003` |
| 25 | 0.54% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 26 | 0.50% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.50% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 28 | 0.46% | `java.util.HashMap.containsKey(Object) line: 603` |
| 29 | 0.46% | `net.minecraft.world.entity.Entity.isPassenger() line: 3733` |
| 30 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |

GC pauses: 0, total 0 ms

