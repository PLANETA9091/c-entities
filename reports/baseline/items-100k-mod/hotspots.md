# JVM hotspots (JFR ExecutionSample)

Samples: 3953

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7008` |
| 2 | 4.98% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 5141` |
| 3 | 4.22% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 4.05% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 3.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7770` |
| 6 | 3.42% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7760` |
| 8 | 2.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12392` |
| 9 | 2.43% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 2.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6553` |
| 11 | 1.77% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 12 | 1.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.16% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12242` |
| 15 | 1.06% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 17 | 0.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4891` |
| 18 | 0.78% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 19 | 0.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6911` |
| 20 | 0.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6955` |
| 21 | 0.66% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 22 | 0.61% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 23 | 0.43% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 24 | 0.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 25 | 0.40% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 26 | 0.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 27 | 0.35% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 28 | 0.35% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 29 | 0.35% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 30 | 0.35% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 31 | 0.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6897` |
| 32 | 0.33% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 33 | 0.30% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 34 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 35 | 0.25% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 36 | 0.25% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 37 | 0.25% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 38 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6767` |
| 39 | 0.23% | `java.util.Arrays.copyOf(Object[], int, Class) line: 3509` |
| 40 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 41 | 0.20% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 42 | 0.20% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 43 | 0.20% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 44 | 0.20% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1916` |
| 45 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 46 | 0.18% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 47 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 48 | 0.18% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 49 | 0.18% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 50 | 0.18% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 51 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 52 | 0.15% | `java.lang.String.hashCode() line: 2433` |
| 53 | 0.15% | `java.util.Objects.equals(Object, Object) line: 62` |
| 54 | 0.15% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 55 | 0.15% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 76` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6740` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 5140` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6758` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7101` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12258` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.00% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 6.55% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 5.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4899` |
| 4 | 5.16% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 5 | 4.22% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 3.42% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5377` |
| 7 | 2.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 1.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4870` |
| 9 | 1.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6897` |
| 10 | 1.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 12 | 1.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7770` |
| 13 | 1.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4894` |
| 14 | 1.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12474` |
| 15 | 0.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6915` |
| 16 | 0.68% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 17 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 18 | 0.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 19 | 0.53% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 20 | 0.53% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 21 | 0.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 22 | 0.38% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 23 | 0.38% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.38% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 25 | 0.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.33% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.33% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 28 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6758` |
| 29 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.28% | `java.util.ArrayList.grow(int) line: 238` |

GC pauses: 0, total 0 ms

