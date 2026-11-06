# JVM hotspots (JFR ExecutionSample)

Samples: 5021

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.25% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10521` |
| 2 | 9.18% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 7.61% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 4.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6120` |
| 5 | 4.36% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4568` |
| 6 | 3.88% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.53% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10512` |
| 8 | 2.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 2.23% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4436` |
| 11 | 1.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6064` |
| 12 | 1.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6055` |
| 13 | 1.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6883` |
| 14 | 0.98% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3237` |
| 15 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6191` |
| 16 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.92% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 18 | 0.80% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10516` |
| 19 | 0.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6893` |
| 20 | 0.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.62% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4516` |
| 22 | 0.58% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3199` |
| 24 | 0.46% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 25 | 0.44% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 26 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6087` |
| 27 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6177` |
| 28 | 0.30% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 30 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 31 | 0.26% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 32 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 33 | 0.24% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 34 | 0.24% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 35 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5866` |
| 36 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 37 | 0.22% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.22% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 39 | 0.22% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 40 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 41 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6065` |
| 42 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 43 | 0.18% | `java.util.Arrays.fill(long[], long) line: 3144` |
| 44 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3190` |
| 45 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3211` |
| 46 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6094` |
| 47 | 0.16% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 48 | 0.16% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.setHeadPlain(MultiThreadedQueue$LinkedNode) line: 41` |
| 49 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5865` |
| 50 | 0.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 51 | 0.16% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6884` |
| 52 | 0.16% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4787` |
| 53 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6005` |
| 54 | 0.16% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4518` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6097` |
| 56 | 0.16% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4567` |
| 57 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 58 | 0.14% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 59 | 0.14% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 60 | 0.14% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 393` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.75% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10704` |
| 2 | 12.87% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 9.18% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 6.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6055` |
| 5 | 4.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4469` |
| 6 | 3.88% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4788` |
| 7 | 2.49% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2559` |
| 8 | 2.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 9 | 2.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6893` |
| 10 | 2.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4333` |
| 11 | 2.09% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.04% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4467` |
| 14 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 0.92% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6177` |
| 16 | 0.68% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4436` |
| 17 | 0.58% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.48% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 19 | 0.46% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4437` |
| 21 | 0.44% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6195` |
| 23 | 0.36% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.26% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 26 | 0.26% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 27 | 0.24% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 28 | 0.22% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 29 | 0.22% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 30 | 0.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10700` |

GC pauses: 0, total 0 ms

