# JVM hotspots (JFR ExecutionSample)

Samples: 4011

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.93% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 7.53% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 3 | 5.44% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 4 | 3.99% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9770` |
| 6 | 3.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 7 | 3.19% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9777` |
| 8 | 2.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 9 | 2.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 2.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2835` |
| 11 | 1.84% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4152` |
| 12 | 1.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 13 | 1.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5684` |
| 14 | 1.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5781` |
| 15 | 1.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9733` |
| 16 | 1.22% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 17 | 1.20% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 18 | 1.12% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2873` |
| 19 | 1.10% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9730` |
| 20 | 1.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5675` |
| 21 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 22 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 23 | 0.90% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.80% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 25 | 0.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 26 | 0.65% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 27 | 0.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5767` |
| 28 | 0.45% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 29 | 0.42% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9738` |
| 30 | 0.32% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 31 | 0.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9953` |
| 32 | 0.30% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 33 | 0.30% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 34 | 0.30% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 35 | 0.27% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 36 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5814` |
| 37 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4065` |
| 38 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 39 | 0.25% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 40 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 41 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 42 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2847` |
| 43 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 44 | 0.20% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 45 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 46 | 0.20% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2825` |
| 48 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5708` |
| 49 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4084` |
| 50 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5625` |
| 51 | 0.17% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 52 | 0.15% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 53 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 54 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 55 | 0.15% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 56 | 0.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 57 | 0.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 58 | 0.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 59 | 0.12% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 60 | 0.12% | `net.minecraft.server.MinecraftServer.runServer() line: 1351` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.02% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9907` |
| 3 | 7.93% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 5.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5675` |
| 5 | 5.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4105` |
| 6 | 4.29% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2217` |
| 7 | 3.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3969` |
| 8 | 3.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 9 | 2.87% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 2.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 2.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4103` |
| 12 | 1.70% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 13 | 1.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.22% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4424` |
| 15 | 1.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5767` |
| 16 | 0.97% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5785` |
| 19 | 0.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4073` |
| 20 | 0.45% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 21 | 0.42% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 22 | 0.40% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.40% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 24 | 0.30% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.30% | `net.minecraft.server.MinecraftServer.lambda$spin$0(AtomicReference) line: 299` |
| 26 | 0.30% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 27 | 0.30% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 28 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.27% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 30 | 0.27% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |

GC pauses: 0, total 0 ms

