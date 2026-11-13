# JVM hotspots (JFR ExecutionSample)

Samples: 5183

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.76% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 4.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11977` |
| 3 | 4.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7346` |
| 4 | 4.09% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4886` |
| 5 | 3.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.20% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.14% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6650` |
| 9 | 1.76% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 10 | 1.60% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.27% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 11827` |
| 12 | 1.20% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6607` |
| 15 | 1.04% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 1.00% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11839` |
| 17 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6285` |
| 18 | 0.62% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 19 | 0.48% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 20 | 0.44% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 21 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 22 | 0.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7356` |
| 23 | 0.37% | `java.util.ArrayList$Itr.next() line: 1058` |
| 24 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 25 | 0.31% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 26 | 0.31% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 27 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6611` |
| 28 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6454` |
| 29 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6593` |
| 30 | 0.29% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 31 | 0.27% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 32 | 0.27% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 33 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 34 | 0.27% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 35 | 0.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11843` |
| 36 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 37 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 38 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 39 | 0.23% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 40 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3460` |
| 41 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 42 | 0.21% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6591` |
| 44 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 45 | 0.17% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 46 | 0.17% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 441` |
| 47 | 0.17% | `java.lang.Double.isFinite(double) line: 1023` |
| 48 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 49 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 50 | 0.15% | `java.lang.StringBuilder.<init>() line: 106` |
| 51 | 0.15% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 215` |
| 52 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3434` |
| 53 | 0.15% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4654` |
| 55 | 0.14% | `java.util.HashMap$HashIterator.nextNode() line: 1610` |
| 56 | 0.14% | `net.minecraft.world.level.levelgen.Heightmap.lambda$static$0(BlockState) line: 28` |
| 57 | 0.14% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 743` |
| 58 | 0.14% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 59 | 0.14% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 60 | 0.14% | `com.mojang.serialization.Encoder$1.encode(Object, DynamicOps, Object) line: 25` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6454` |
| 2 | 7.22% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 5.04% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 4.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6536` |
| 5 | 4.80% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 4.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4654` |
| 7 | 3.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 3.65% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 2.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4626` |
| 10 | 2.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6593` |
| 12 | 1.60% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5122` |
| 13 | 1.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12072` |
| 14 | 1.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12059` |
| 15 | 1.04% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.95% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2683` |
| 17 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 18 | 0.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4649` |
| 19 | 0.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 20 | 0.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7356` |
| 21 | 0.44% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.44% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.42% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 24 | 0.39% | `java.util.ArrayList.grow(int) line: 238` |
| 25 | 0.39% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.33% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 163` |
| 27 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.27% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 30 | 0.27% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 474` |

GC pauses: 0, total 0 ms

