# JVM hotspots (JFR ExecutionSample)

Samples: 4935

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 10.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5473` |
| 3 | 6.38% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4054` |
| 5 | 2.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 6 | 1.52% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3927` |
| 8 | 1.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5478` |
| 9 | 1.24% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 10 | 1.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5549` |
| 11 | 1.16% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.11% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9470` |
| 13 | 1.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.09% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9473` |
| 15 | 1.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6241` |
| 16 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 17 | 0.81% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4007` |
| 18 | 0.79% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2760` |
| 19 | 0.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2722` |
| 20 | 0.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6251` |
| 21 | 0.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5534` |
| 22 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 23 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9510` |
| 24 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5378` |
| 25 | 0.32% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 26 | 0.30% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 27 | 0.30% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 28 | 0.30% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 29 | 0.28% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 30 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 31 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2713` |
| 32 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5420` |
| 33 | 0.28% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9478` |
| 34 | 0.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2705` |
| 35 | 0.26% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4051` |
| 36 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 37 | 0.24% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2734` |
| 39 | 0.24% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 40 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 41 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 42 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 43 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 44 | 0.22% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 45 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5421` |
| 46 | 0.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9689` |
| 47 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 48 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 49 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3831` |
| 50 | 0.20% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4009` |
| 51 | 0.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6240` |
| 52 | 0.18% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 53 | 0.18% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 54 | 0.18% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2719` |
| 56 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 57 | 0.18% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 58 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5466` |
| 59 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5289` |
| 60 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5532` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.18% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 8.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9642` |
| 3 | 6.40% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3960` |
| 5 | 3.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3824` |
| 6 | 3.18% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2104` |
| 7 | 3.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9652` |
| 8 | 2.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6251` |
| 9 | 2.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 2.11% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 2.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5534` |
| 13 | 1.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3958` |
| 14 | 1.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 1.07% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 16 | 1.03% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3927` |
| 17 | 0.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 18 | 0.45% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 19 | 0.41% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.41% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 21 | 0.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 22 | 0.39% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 23 | 0.34% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 24 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3928` |
| 25 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 26 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.24% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 29 | 0.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.24% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |

GC pauses: 0, total 0 ms

