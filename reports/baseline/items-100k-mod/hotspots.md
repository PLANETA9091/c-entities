# JVM hotspots (JFR ExecutionSample)

Samples: 3856

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.17% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 6.59% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4886` |
| 3 | 5.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11977` |
| 4 | 4.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7346` |
| 6 | 2.80% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6463` |
| 8 | 2.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 9 | 1.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6285` |
| 10 | 1.63% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7356` |
| 12 | 1.40% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 11827` |
| 13 | 1.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6289` |
| 15 | 0.93% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 16 | 0.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11839` |
| 17 | 0.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 18 | 0.91% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 19 | 0.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6607` |
| 20 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11843` |
| 22 | 0.60% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 23 | 0.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4646` |
| 24 | 0.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6486` |
| 25 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 26 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3460` |
| 27 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6636` |
| 28 | 0.36% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.36% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 30 | 0.29% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 31 | 0.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3434` |
| 32 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6454` |
| 33 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6593` |
| 34 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 35 | 0.23% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 37 | 0.23% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 38 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6650` |
| 39 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4654` |
| 40 | 0.23% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 41 | 0.21% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 42 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6437` |
| 43 | 0.18% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 44 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 45 | 0.18% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 46 | 0.18% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 30` |
| 47 | 0.18% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 48 | 0.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 49 | 0.16% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 50 | 0.16% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 51 | 0.16% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 52 | 0.16% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 215` |
| 53 | 0.16% | `java.util.ArrayList$Itr.next() line: 1058` |
| 54 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6591` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11878` |
| 56 | 0.16% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 510` |
| 57 | 0.16% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 58 | 0.13% | `java.util.stream.ReferencePipeline$3$1.accept(Object) line: 214` |
| 59 | 0.13% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 60 | 0.13% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6454` |
| 2 | 9.62% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 6.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4654` |
| 4 | 5.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 4.95% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 4.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4626` |
| 7 | 3.94% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.80% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.84% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12072` |
| 10 | 1.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4649` |
| 11 | 1.63% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5122` |
| 12 | 1.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12059` |
| 13 | 1.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7356` |
| 15 | 1.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6536` |
| 16 | 1.12% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2683` |
| 17 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6593` |
| 18 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.60% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 21 | 0.47% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 23 | 0.39% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.39% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 25 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6611` |
| 26 | 0.34% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.29% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 28 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.29% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 30 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

