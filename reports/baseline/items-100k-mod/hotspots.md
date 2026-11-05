# JVM hotspots (JFR ExecutionSample)

Samples: 5612

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.42% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10314` |
| 2 | 7.75% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 5.58% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 5.04% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6021` |
| 5 | 3.76% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4469` |
| 6 | 2.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.07% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10305` |
| 8 | 2.05% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 2.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 11 | 1.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6092` |
| 12 | 1.07% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 13 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5965` |
| 15 | 0.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 16 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.77% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3138` |
| 18 | 0.77% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3100` |
| 19 | 0.75% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10309` |
| 20 | 0.59% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4417` |
| 21 | 0.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6794` |
| 22 | 0.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6078` |
| 23 | 0.50% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 24 | 0.46% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 25 | 0.39% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 26 | 0.39% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 27 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5767` |
| 28 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3091` |
| 29 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5903` |
| 30 | 0.30% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 31 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 32 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 33 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 34 | 0.29% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 35 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5859` |
| 36 | 0.25% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 393` |
| 37 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5766` |
| 38 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5906` |
| 39 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 40 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 41 | 0.23% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 42 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3112` |
| 43 | 0.23% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10344` |
| 44 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 45 | 0.21% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 46 | 0.21% | `java.util.Arrays.fill(long[], long) line: 3144` |
| 47 | 0.21% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 48 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5860` |
| 49 | 0.20% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 50 | 0.20% | `java.lang.Double.isFinite(double) line: 1023` |
| 51 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5941` |
| 52 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5988` |
| 53 | 0.18% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 54 | 0.18% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 55 | 0.18% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 56 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 135` |
| 57 | 0.18% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 76` |
| 58 | 0.18% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 59 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5768` |
| 60 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5997` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10497` |
| 2 | 14.33% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.77% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 5 | 4.03% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4370` |
| 6 | 2.99% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2482` |
| 7 | 2.48% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4234` |
| 8 | 2.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6794` |
| 9 | 2.05% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4689` |
| 10 | 2.01% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6078` |
| 14 | 0.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4368` |
| 15 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.61% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 17 | 0.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 19 | 0.41% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.39% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 21 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4338` |
| 22 | 0.37% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 23 | 0.37% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 24 | 0.32% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.29% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 26 | 0.29% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 27 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.util.ParallelSearchRadiusIteration.generateBFSOrder(int) line: 344` |
| 28 | 0.25% | `net.minecraft.world.level.chunk.status.ChunkStatusTasks.lambda$buildTerrain$0(ChunkAccess) line: 130` |
| 29 | 0.25% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 30 | 0.23% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |

GC pauses: 0, total 0 ms

