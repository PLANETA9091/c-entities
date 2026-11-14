# JVM hotspots (JFR ExecutionSample)

Samples: 5523

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7056` |
| 2 | 5.21% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 5.09% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7808` |
| 4 | 4.82% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 5185` |
| 5 | 4.51% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 4.06% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 2.90% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.83% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 9 | 1.74% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12290` |
| 10 | 1.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12440` |
| 11 | 1.14% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6597` |
| 13 | 1.10% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6959` |
| 17 | 0.71% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12425` |
| 18 | 0.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7818` |
| 19 | 0.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7003` |
| 20 | 0.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4935` |
| 21 | 0.54% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 22 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 23 | 0.53% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 24 | 0.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 25 | 0.42% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2447` |
| 26 | 0.31% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 27 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 28 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 29 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6802` |
| 30 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 31 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 32 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 33 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 34 | 0.20% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 35 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 135` |
| 36 | 0.18% | `java.util.ArrayList$Itr.next() line: 1058` |
| 37 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 38 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 39 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 40 | 0.16% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 41 | 0.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 42 | 0.16% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 43 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4867` |
| 44 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4948` |
| 45 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6834` |
| 46 | 0.14% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 47 | 0.14% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 76` |
| 48 | 0.14% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 49 | 0.14% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 50 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 51 | 0.13% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 52 | 0.13% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 53 | 0.13% | `java.lang.Math.unsignedMultiplyHigh(long, long) line: 1433` |
| 54 | 0.13% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 55 | 0.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4913` |
| 56 | 0.13% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 180` |
| 57 | 0.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4905` |
| 58 | 0.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6785` |
| 59 | 0.13% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 510` |
| 60 | 0.13% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.74% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 5.94% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 5.21% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 5.03% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4943` |
| 5 | 4.65% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 4.51% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5421` |
| 7 | 2.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4914` |
| 8 | 2.26% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 9 | 1.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6945` |
| 11 | 1.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12522` |
| 12 | 1.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7818` |
| 13 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 0.81% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4938` |
| 15 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6946` |
| 17 | 0.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 18 | 0.42% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 0.42% | `java.util.ArrayList.grow(int) line: 238` |
| 20 | 0.42% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 21 | 0.38% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.33% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 23 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 26 | 0.25% | `net.minecraft.world.level.chunk.status.ChunkStatusTasks.lambda$buildTerrain$0(ChunkAccess) line: 130` |
| 27 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7017` |
| 28 | 0.24% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 29 | 0.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12541` |
| 30 | 0.22% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |

GC pauses: 0, total 0 ms

