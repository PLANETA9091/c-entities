# JVM hotspots (JFR ExecutionSample)

Samples: 4116

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8042` |
| 2 | 7.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4360` |
| 3 | 5.83% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 5.64% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.49% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3055` |
| 6 | 4.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7989` |
| 7 | 3.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2947` |
| 8 | 2.77% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 9 | 2.65% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 2.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5080` |
| 11 | 1.92% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3008` |
| 12 | 1.51% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4366` |
| 15 | 1.00% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 0.92% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7993` |
| 17 | 0.92% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 18 | 0.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.78% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7683` |
| 21 | 0.70% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2195` |
| 22 | 0.68% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8003` |
| 23 | 0.66% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7685` |
| 24 | 0.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4405` |
| 25 | 0.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4397` |
| 26 | 0.56% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 27 | 0.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5078` |
| 28 | 0.44% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 29 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2176` |
| 30 | 0.41% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1868` |
| 31 | 0.39% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 32 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 33 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 34 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 35 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8000` |
| 36 | 0.32% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 37 | 0.32% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 38 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 39 | 0.32% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 40 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 41 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4308` |
| 42 | 0.24% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 43 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 44 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4429` |
| 45 | 0.24% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 906` |
| 46 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4221` |
| 47 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2940` |
| 48 | 0.22% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 49 | 0.22% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 50 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 51 | 0.19% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 52 | 0.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 53 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 54 | 0.17% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7971` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4309` |
| 57 | 0.17% | `java.lang.Double.isFinite(double) line: 1023` |
| 58 | 0.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 59 | 0.17% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7972` |
| 60 | 0.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8035` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.19% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 8.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8038` |
| 4 | 6.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4397` |
| 5 | 4.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2968` |
| 6 | 4.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5080` |
| 7 | 2.77% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 8 | 2.75% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 2.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2850` |
| 10 | 2.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8030` |
| 11 | 2.33% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7996` |
| 12 | 2.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2967` |
| 13 | 1.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 14 | 1.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.41% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1709` |
| 16 | 1.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2947` |
| 17 | 1.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 18 | 1.00% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.92% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2948` |
| 20 | 0.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 21 | 0.83% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7988` |
| 22 | 0.58% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 548` |
| 23 | 0.46% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 24 | 0.44% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7989` |
| 26 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.36% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 28 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.32% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |
| 30 | 0.32% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |

GC pauses: 0, total 0 ms

