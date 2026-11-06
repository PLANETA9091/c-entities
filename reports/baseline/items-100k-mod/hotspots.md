# JVM hotspots (JFR ExecutionSample)

Samples: 5259

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.82% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10491` |
| 2 | 7.34% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 4.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6120` |
| 4 | 4.68% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 3.92% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4568` |
| 6 | 2.99% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.94% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10482` |
| 8 | 1.90% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4436` |
| 11 | 1.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6191` |
| 12 | 1.43% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 13 | 1.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6055` |
| 14 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 1.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6064` |
| 16 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.76% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3199` |
| 18 | 0.76% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10486` |
| 19 | 0.72% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4516` |
| 20 | 0.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6893` |
| 21 | 0.59% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.57% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3237` |
| 23 | 0.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6177` |
| 24 | 0.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6883` |
| 25 | 0.48% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 26 | 0.42% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 27 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5958` |
| 28 | 0.40% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 29 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5866` |
| 30 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 31 | 0.34% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 32 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 33 | 0.32% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 34 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 35 | 0.29% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 36 | 0.27% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 37 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6002` |
| 38 | 0.25% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 39 | 0.25% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 40 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5865` |
| 41 | 0.25% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 42 | 0.25% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 43 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 44 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 45 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 46 | 0.23% | `java.util.Arrays.fill(long[], long) line: 3144` |
| 47 | 0.21% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 393` |
| 48 | 0.21% | `java.lang.Double.isFinite(double) line: 1023` |
| 49 | 0.21% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 50 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6175` |
| 51 | 0.21% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 52 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6087` |
| 53 | 0.19% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 54 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3190` |
| 55 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6190` |
| 56 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5867` |
| 57 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6224` |
| 58 | 0.19% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4787` |
| 59 | 0.17% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 60 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.27% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 13.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10674` |
| 3 | 7.34% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4469` |
| 5 | 3.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6055` |
| 6 | 2.95% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4333` |
| 7 | 2.41% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2559` |
| 8 | 2.00% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 1.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6893` |
| 10 | 1.90% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4788` |
| 11 | 1.86% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6177` |
| 13 | 1.35% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 14 | 1.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4467` |
| 15 | 0.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4436` |
| 16 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.51% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.51% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 20 | 0.48% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4437` |
| 21 | 0.44% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 22 | 0.40% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 23 | 0.38% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.34% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 25 | 0.32% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.32% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 27 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.29% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 29 | 0.25% | `java.util.ArrayList.grow(int) line: 238` |
| 30 | 0.25% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.light(LightChunkGetter, ChunkAccess, Boolean[]) line: 914` |

GC pauses: 0, total 0 ms

