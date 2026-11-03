# JVM hotspots (JFR ExecutionSample)

Samples: 4009

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.56% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 5.59% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 4.07% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.02% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 5 | 3.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 6 | 3.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 7 | 2.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 8 | 2.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9789` |
| 9 | 2.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5684` |
| 10 | 2.10% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9796` |
| 11 | 1.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5781` |
| 12 | 1.70% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9749` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9752` |
| 14 | 1.25% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 15 | 1.22% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4152` |
| 16 | 1.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 17 | 1.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5675` |
| 18 | 1.05% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2873` |
| 19 | 0.95% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 20 | 0.95% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 21 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 22 | 0.82% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2835` |
| 23 | 0.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 24 | 0.70% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 25 | 0.67% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 26 | 0.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 27 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2847` |
| 28 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9757` |
| 29 | 0.35% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 30 | 0.32% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 31 | 0.32% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 32 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5767` |
| 33 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5488` |
| 34 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5489` |
| 35 | 0.30% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 36 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 37 | 0.30% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 38 | 0.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2832` |
| 39 | 0.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2826` |
| 40 | 0.25% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 41 | 0.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2825` |
| 42 | 0.25% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4147` |
| 43 | 0.25% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 44 | 0.25% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 45 | 0.22% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 46 | 0.22% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 47 | 0.22% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.getTailOpaque() line: 80` |
| 48 | 0.22% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 49 | 0.22% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 50 | 0.22% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4154` |
| 51 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 52 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 53 | 0.20% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 54 | 0.20% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 55 | 0.20% | `net.minecraft.world.level.chunk.LevelChunk.getListenerRegistry(int) line: 272` |
| 56 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 57 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 954` |
| 58 | 0.17% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.17% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 60 | 0.17% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.95% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 7.93% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9926` |
| 3 | 6.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5675` |
| 4 | 5.59% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 5 | 4.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4105` |
| 6 | 4.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 7 | 3.94% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3969` |
| 8 | 3.64% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2217` |
| 9 | 3.14% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 2.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 11 | 2.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4103` |
| 12 | 1.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.67% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 14 | 1.25% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4424` |
| 15 | 1.07% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 0.95% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4073` |
| 17 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5767` |
| 19 | 0.67% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 20 | 0.50% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.50% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 22 | 0.45% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.45% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 24 | 0.37% | `java.math.BigInteger.multiplyToLen(int[], int, int[], int, int[]) line: 1844` |
| 25 | 0.35% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 26 | 0.32% | `net.minecraft.server.MinecraftServer.lambda$spin$0(AtomicReference) line: 299` |
| 27 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.30% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 29 | 0.30% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 30 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5785` |

GC pauses: 0, total 0 ms

