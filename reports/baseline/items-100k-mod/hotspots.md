# JVM hotspots (JFR ExecutionSample)

Samples: 4588

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.10% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 5.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.86% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4916` |
| 4 | 4.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6728` |
| 5 | 4.21% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7473` |
| 8 | 2.77% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 2.33% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 10 | 1.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6315` |
| 11 | 1.57% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 11955` |
| 12 | 1.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6657` |
| 13 | 1.24% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 1.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12105` |
| 15 | 0.96% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7483` |
| 18 | 0.70% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 19 | 0.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6641` |
| 20 | 0.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1299` |
| 22 | 0.41% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 23 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6643` |
| 24 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6536` |
| 25 | 0.31% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 26 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 27 | 0.31% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 28 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 29 | 0.28% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 30 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6661` |
| 31 | 0.24% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 32 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 33 | 0.24% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 34 | 0.24% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 35 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6504` |
| 36 | 0.22% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 37 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 38 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 39 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 40 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 41 | 0.22% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 42 | 0.20% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 43 | 0.20% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 44 | 0.20% | `java.lang.Double.isFinite(double) line: 1023` |
| 45 | 0.20% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 46 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 47 | 0.17% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 48 | 0.17% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 49 | 0.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 50 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 51 | 0.17% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 52 | 0.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 53 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 534` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6487` |
| 55 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 56 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 57 | 0.13% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 58 | 0.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4615` |
| 59 | 0.13% | `ru.planeta9091.cebaseline.BaselinePlugin.onTickEnd(ServerTickEndEvent) line: 546` |
| 60 | 0.11% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.88% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 8.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6504` |
| 3 | 5.21% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 4 | 5.14% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 5.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4684` |
| 6 | 5.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6586` |
| 7 | 3.77% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 3.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4656` |
| 9 | 2.77% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5152` |
| 10 | 2.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6643` |
| 11 | 1.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12187` |
| 12 | 1.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.37% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 14 | 1.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 1.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4679` |
| 16 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.65% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 18 | 0.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7483` |
| 19 | 0.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12248` |
| 20 | 0.52% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 21 | 0.48% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 23 | 0.39% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.33% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.33% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |
| 27 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6661` |
| 28 | 0.28% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 29 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 30 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |

GC pauses: 0, total 0 ms

