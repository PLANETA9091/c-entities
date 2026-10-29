# JVM hotspots (JFR ExecutionSample)

Samples: 4022

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8102` |
| 2 | 8.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4420` |
| 3 | 7.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 4.62% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3115` |
| 5 | 4.48% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 4.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8049` |
| 7 | 3.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3007` |
| 8 | 3.43% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.44% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3068` |
| 10 | 1.99% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 11 | 1.86% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2228` |
| 12 | 1.84% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5140` |
| 14 | 1.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4426` |
| 15 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 16 | 1.14% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.97% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2247` |
| 18 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 19 | 0.80% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 20 | 0.72% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8053` |
| 21 | 0.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 22 | 0.57% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 23 | 0.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4457` |
| 24 | 0.50% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7745` |
| 25 | 0.47% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7743` |
| 26 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5138` |
| 27 | 0.42% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1891` |
| 28 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8060` |
| 29 | 0.40% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 30 | 0.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4465` |
| 31 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 32 | 0.32% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 33 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8063` |
| 34 | 0.30% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 929` |
| 35 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.27% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3063` |
| 37 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 38 | 0.25% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 39 | 0.25% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 40 | 0.22% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 41 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4489` |
| 42 | 0.20% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 43 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 44 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2912` |
| 45 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3019` |
| 46 | 0.20% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 47 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 48 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 49 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2223` |
| 50 | 0.17% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3070` |
| 51 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4431` |
| 52 | 0.15% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 53 | 0.15% | `java.util.Comparator.lambda$comparingLong$6043328a$1(ToLongFunction, Object, Object) line: 515` |
| 54 | 0.15% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 55 | 0.15% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2231` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 276` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3000` |
| 59 | 0.12% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 60 | 0.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.20% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 7.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4457` |
| 4 | 6.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8098` |
| 5 | 5.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5140` |
| 6 | 4.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3028` |
| 7 | 3.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2910` |
| 8 | 3.41% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1732` |
| 9 | 3.06% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3027` |
| 10 | 2.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8090` |
| 11 | 2.06% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.99% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 13 | 1.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3007` |
| 14 | 1.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8056` |
| 15 | 1.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 16 | 1.29% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 17 | 1.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 1.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3008` |
| 20 | 0.60% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8048` |
| 21 | 0.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 22 | 0.55% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 559` |
| 23 | 0.45% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 24 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2232` |
| 25 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8049` |
| 26 | 0.35% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 27 | 0.32% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 28 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.27% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 30 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

GC pauses: 0, total 0 ms

