# JVM hotspots (JFR ExecutionSample)

Samples: 5800

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.97% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 2 | 10.03% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 9.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5471` |
| 4 | 4.12% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4052` |
| 5 | 3.88% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 1.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 7 | 1.64% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 1.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5476` |
| 9 | 1.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9388` |
| 10 | 1.12% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 11 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 0.95% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9385` |
| 13 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3925` |
| 15 | 0.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5547` |
| 16 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2758` |
| 17 | 0.79% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2720` |
| 18 | 0.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6249` |
| 19 | 0.69% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4005` |
| 20 | 0.64% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 21 | 0.62% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 22 | 0.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5532` |
| 23 | 0.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6239` |
| 24 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2710` |
| 25 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9393` |
| 26 | 0.36% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 27 | 0.36% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9425` |
| 28 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 29 | 0.33% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 30 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3918` |
| 31 | 0.31% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 32 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 33 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5288` |
| 34 | 0.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2732` |
| 35 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5377` |
| 36 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 37 | 0.22% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5418` |
| 39 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5580` |
| 40 | 0.21% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 41 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 42 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 43 | 0.21% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.setBoolean(Object, boolean) line: 132` |
| 44 | 0.21% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 45 | 0.21% | `java.lang.Double.isFinite(double) line: 1023` |
| 46 | 0.21% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 47 | 0.21% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 48 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5376` |
| 49 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5417` |
| 50 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5287` |
| 51 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 52 | 0.19% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 53 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2711` |
| 54 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5420` |
| 55 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3829` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2703` |
| 58 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5475` |
| 59 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5289` |
| 60 | 0.16% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.38% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 10.97% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9433` |
| 3 | 10.05% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3958` |
| 5 | 3.48% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2102` |
| 6 | 3.09% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9502` |
| 7 | 2.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9492` |
| 8 | 1.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6249` |
| 9 | 1.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.47% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3822` |
| 13 | 1.14% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2733` |
| 14 | 1.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5532` |
| 15 | 1.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3956` |
| 16 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3925` |
| 19 | 0.41% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.41% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 21 | 0.38% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.36% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 23 | 0.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9501` |
| 24 | 0.31% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 25 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.26% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.buildSurface(ChunkAccess, NoiseChunk, RandomState, BiomeManager, Set, MaterialRule, ServerLevel) line: 227` |
| 27 | 0.26% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.24% | `net.minecraft.util.StaticCache2D.lambda$map$0(StaticCache2D$MappingFunction, int, int) line: 36` |
| 29 | 0.24% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 30 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |

GC pauses: 0, total 0 ms

