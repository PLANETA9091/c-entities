# JVM hotspots (JFR ExecutionSample)

Samples: 3235

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 6.03% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.94% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 4 | 3.31% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 3.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 6 | 3.15% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4152` |
| 7 | 2.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5684` |
| 8 | 2.16% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 9 | 2.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5781` |
| 10 | 2.10% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2835` |
| 11 | 1.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 12 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 1.64% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 14 | 1.55% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 15 | 1.45% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9770` |
| 16 | 1.36% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9777` |
| 17 | 1.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2873` |
| 18 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 19 | 0.99% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 20 | 0.96% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 21 | 0.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5675` |
| 22 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 23 | 0.77% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5814` |
| 25 | 0.68% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9730` |
| 26 | 0.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 27 | 0.49% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 28 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9733` |
| 29 | 0.37% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 30 | 0.31% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 31 | 0.31% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4147` |
| 32 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5767` |
| 33 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 34 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 35 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 36 | 0.28% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4148` |
| 37 | 0.28% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9738` |
| 38 | 0.28% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4154` |
| 39 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 40 | 0.25% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 41 | 0.25% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 42 | 0.25% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 43 | 0.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 44 | 0.25% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 45 | 0.25% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 46 | 0.22% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.22% | `java.lang.Double.isFinite(double) line: 1023` |
| 48 | 0.22% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 49 | 0.19% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2847` |
| 51 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2826` |
| 52 | 0.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9953` |
| 53 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 54 | 0.19% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4149` |
| 55 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5581` |
| 56 | 0.19% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 57 | 0.19% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 58 | 0.15% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1863` |
| 59 | 0.15% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 60 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 743` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.25% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 6.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5675` |
| 3 | 6.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4105` |
| 4 | 5.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3969` |
| 5 | 4.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2217` |
| 6 | 4.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9907` |
| 7 | 4.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4103` |
| 8 | 3.89% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 9 | 3.37% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 10 | 2.87% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 2.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 12 | 1.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.67% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5767` |
| 15 | 1.55% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4073` |
| 16 | 0.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5785` |
| 17 | 0.96% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4424` |
| 18 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.53% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 21 | 0.49% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.43% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.37% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.37% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 25 | 0.37% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.31% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 27 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 29 | 0.28% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 30 | 0.28% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |

GC pauses: 0, total 0 ms

