# JVM hotspots (JFR ExecutionSample)

Samples: 4057

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.85% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 5.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11978` |
| 3 | 4.98% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4887` |
| 4 | 4.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7347` |
| 5 | 3.85% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.18% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6464` |
| 9 | 1.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6286` |
| 10 | 1.77% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.36% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 11828` |
| 12 | 1.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6290` |
| 13 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.91% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 0.89% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 16 | 0.89% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11840` |
| 17 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 18 | 0.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6608` |
| 19 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.59% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 21 | 0.54% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3461` |
| 22 | 0.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7357` |
| 23 | 0.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6607` |
| 24 | 0.44% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 25 | 0.37% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 26 | 0.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 27 | 0.35% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 28 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4647` |
| 29 | 0.32% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 30 | 0.32% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 31 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6455` |
| 32 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 33 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 34 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11844` |
| 35 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 37 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6537` |
| 38 | 0.22% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 441` |
| 39 | 0.22% | `dev.planeta9091.centities.CeHooks.nativeMergeScan(int, double, double, double, double, double, long, long[])` |
| 40 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6487` |
| 41 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 42 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 43 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 44 | 0.20% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 45 | 0.20% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 46 | 0.20% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6218` |
| 48 | 0.17% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 49 | 0.17% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 50 | 0.17% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 51 | 0.17% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6651` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6594` |
| 54 | 0.17% | `java.util.ArrayList$Itr.next() line: 1058` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3435` |
| 56 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 57 | 0.15% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 58 | 0.15% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 59 | 0.15% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 60 | 0.15% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6455` |
| 2 | 9.66% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 5.96% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 5.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4655` |
| 5 | 4.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 3.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 3.80% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.81% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4627` |
| 9 | 2.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6537` |
| 10 | 1.77% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5123` |
| 11 | 1.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4650` |
| 12 | 1.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12073` |
| 14 | 1.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12060` |
| 15 | 1.26% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2684` |
| 16 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7357` |
| 18 | 0.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 19 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6594` |
| 22 | 0.49% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 23 | 0.49% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 24 | 0.44% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.42% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 26 | 0.32% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 27 | 0.30% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.30% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6637` |
| 30 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

