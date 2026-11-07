# JVM hotspots (JFR ExecutionSample)

Samples: 4963

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.76% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 7.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 3 | 5.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10935` |
| 4 | 4.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 4.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 4.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6309` |
| 7 | 3.95% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 3.36% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4729` |
| 9 | 2.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4597` |
| 10 | 1.93% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.81% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6380` |
| 13 | 1.21% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 10785` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6236` |
| 15 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 1.01% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3366` |
| 19 | 0.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7082` |
| 20 | 0.81% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4677` |
| 21 | 0.79% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 22 | 0.77% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10797` |
| 23 | 0.73% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 24 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10801` |
| 25 | 0.40% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 26 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6259` |
| 27 | 0.32% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 28 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 29 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 30 | 0.30% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 409` |
| 31 | 0.28% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 32 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 33 | 0.28% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 34 | 0.26% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 35 | 0.26% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10806` |
| 36 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6366` |
| 37 | 0.26% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 393` |
| 38 | 0.26% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 39 | 0.18% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 40 | 0.18% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 41 | 0.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 42 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6227` |
| 43 | 0.18% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 44 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6038` |
| 45 | 0.18% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 508` |
| 46 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 47 | 0.16% | `java.util.ArrayList$Itr.next() line: 1058` |
| 48 | 0.16% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 49 | 0.16% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 50 | 0.16% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4728` |
| 51 | 0.16% | `jdk.internal.foreign.HeapMemorySegmentImpl$OfByte.unsafeGetBase() line: 114` |
| 52 | 0.14% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 53 | 0.14% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 54 | 0.14% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 55 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 56 | 0.14% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 76` |
| 57 | 0.14% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3318` |
| 58 | 0.14% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3317` |
| 59 | 0.14% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 60 | 0.14% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.81% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 7.96% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 6.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6227` |
| 4 | 6.13% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 5.50% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 4.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 3.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4630` |
| 8 | 3.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4494` |
| 9 | 2.64% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7082` |
| 10 | 2.14% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2619` |
| 11 | 1.93% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4949` |
| 12 | 1.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11030` |
| 13 | 1.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11017` |
| 15 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 16 | 1.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4628` |
| 17 | 0.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4597` |
| 18 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6366` |
| 20 | 0.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 21 | 0.58% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3345` |
| 22 | 0.54% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 23 | 0.44% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4598` |
| 25 | 0.38% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 442` |
| 26 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.30% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 29 | 0.28% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 30 | 0.28% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |

GC pauses: 0, total 0 ms

