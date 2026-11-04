# JVM hotspots (JFR ExecutionSample)

Samples: 5182

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.67% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 6.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5978` |
| 3 | 5.73% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10094` |
| 4 | 4.09% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 3.61% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4467` |
| 6 | 2.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 2.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10047` |
| 9 | 1.72% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10050` |
| 10 | 1.62% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10087` |
| 11 | 1.39% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 12 | 1.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6049` |
| 13 | 1.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5943` |
| 14 | 0.96% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 15 | 0.89% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3136` |
| 16 | 0.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4335` |
| 17 | 0.89% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6751` |
| 18 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5952` |
| 20 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3098` |
| 22 | 0.69% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6741` |
| 23 | 0.64% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10055` |
| 25 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6035` |
| 26 | 0.41% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4415` |
| 27 | 0.37% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 28 | 0.35% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 29 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 30 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5756` |
| 31 | 0.31% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 32 | 0.31% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 33 | 0.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3089` |
| 34 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 35 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 36 | 0.29% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 37 | 0.29% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 38 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5757` |
| 39 | 0.27% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 40 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 41 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6031` |
| 42 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5890` |
| 43 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 44 | 0.25% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 45 | 0.25% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 46 | 0.25% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5891` |
| 48 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3088` |
| 49 | 0.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10270` |
| 50 | 0.21% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 51 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5758` |
| 52 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4239` |
| 53 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 54 | 0.19% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 55 | 0.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 56 | 0.19% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 57 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5892` |
| 58 | 0.19% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4417` |
| 59 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 60 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.61% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10224` |
| 3 | 9.71% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 3.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4368` |
| 5 | 3.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5943` |
| 6 | 3.11% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2480` |
| 7 | 2.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4232` |
| 8 | 2.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 9 | 2.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6751` |
| 12 | 1.39% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4687` |
| 13 | 0.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6035` |
| 14 | 0.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4366` |
| 15 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.50% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 18 | 0.41% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4335` |
| 20 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6053` |
| 21 | 0.35% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.33% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.29% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 24 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 25 | 0.29% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.29% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 27 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4336` |
| 28 | 0.29% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 29 | 0.29% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 30 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

