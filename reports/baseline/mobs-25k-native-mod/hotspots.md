# JVM hotspots (JFR ExecutionSample)

Samples: 2082

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.38% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13003` |
| 2 | 4.37% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.88% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.35% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 5 | 2.16% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 2.07% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.73% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13245` |
| 12 | 1.15% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 13 | 1.10% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8335` |
| 15 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 16 | 0.82% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13022` |
| 17 | 0.77% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 18 | 0.72% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 19 | 0.72% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 20 | 0.72% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 21 | 0.72% | `java.util.HashMap.getNode(Object) line: 586` |
| 22 | 0.62% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 23 | 0.58% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 24 | 0.58% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1927` |
| 25 | 0.58% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 26 | 0.58% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 27 | 0.53% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 28 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 29 | 0.38% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 30 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 31 | 0.38% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 32 | 0.38% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 33 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 34 | 0.38% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 35 | 0.38% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 36 | 0.38% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 37 | 0.38% | `net.minecraft.world.entity.EntityDimensions.makeBoundingBox(double, double, double) line: 22` |
| 38 | 0.38% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11227` |
| 39 | 0.34% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 40 | 0.34% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 41 | 0.34% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 42 | 0.34% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 43 | 0.34% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4084` |
| 44 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13040` |
| 45 | 0.34% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 46 | 0.29% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 47 | 0.29% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 48 | 0.29% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 49 | 0.29% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 50 | 0.29% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 51 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 52 | 0.24% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 53 | 0.24% | `jdk.internal.classfile.impl.EntryMap.firstToken(int) line: 92` |
| 54 | 0.24% | `java.util.HashMap.getNode(Object) line: 579` |
| 55 | 0.24% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 56 | 0.24% | `java.lang.String.length() line: 1583` |
| 57 | 0.24% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 58 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.task.ChunkLoadTask.<init>(ChunkTaskScheduler, ServerLevel, int, int, NewChunkHolder, Priority) line: 49` |
| 59 | 0.24% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 76` |
| 60 | 0.24% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13245` |
| 2 | 4.23% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11244` |
| 3 | 3.94% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.93% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11177` |
| 5 | 2.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 6 | 2.11% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 7 | 2.07% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.87% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.78% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 10 | 1.63% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6858` |
| 13 | 1.30% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 14 | 1.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8361` |
| 15 | 1.15% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 16 | 1.06% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 17 | 0.96% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11205` |
| 18 | 0.91% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11197` |
| 19 | 0.86% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 20 | 0.82% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5771` |
| 21 | 0.77% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 22 | 0.62% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |
| 23 | 0.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 24 | 0.58% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.58% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 26 | 0.53% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.53% | `java.util.HashMap.containsKey(Object) line: 603` |
| 28 | 0.43% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 30 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms

