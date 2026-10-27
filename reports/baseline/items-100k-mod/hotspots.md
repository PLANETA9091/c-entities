# JVM hotspots (JFR ExecutionSample)

Samples: 3635

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.83% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 2 | 7.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 3 | 5.09% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2410` |
| 5 | 3.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 2.75% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2335` |
| 8 | 2.20% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 9 | 2.06% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2381` |
| 10 | 2.04% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 11 | 1.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 13 | 1.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.99% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 16 | 0.96% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.91% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7413` |
| 18 | 0.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2421` |
| 19 | 0.83% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1715` |
| 20 | 0.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4524` |
| 21 | 0.61% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 22 | 0.61% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2519` |
| 23 | 0.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3813` |
| 24 | 0.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 25 | 0.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2326` |
| 26 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3850` |
| 27 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1696` |
| 28 | 0.36% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 29 | 0.36% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 30 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 31 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2339` |
| 32 | 0.30% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 33 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 34 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2327` |
| 35 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2367` |
| 36 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 37 | 0.22% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 38 | 0.22% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5195` |
| 39 | 0.22% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 40 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2345` |
| 41 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 42 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 43 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3858` |
| 44 | 0.19% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 45 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3875` |
| 46 | 0.17% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 47 | 0.17% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 48 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 49 | 0.17% | `java.lang.Class.cast(Object) line: 3491` |
| 50 | 0.17% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 76` |
| 51 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1699` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 640` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3819` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2400` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7420` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2319` |
| 58 | 0.14% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 59 | 0.14% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 60 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7458` |
| 2 | 11.69% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4524` |
| 3 | 7.59% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3850` |
| 5 | 2.81% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2324` |
| 6 | 2.56% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.39% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7416` |
| 8 | 2.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 1.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7450` |
| 10 | 1.95% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2335` |
| 12 | 1.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 13 | 1.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2336` |
| 14 | 1.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1307` |
| 16 | 0.96% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.91% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 18 | 0.69% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7408` |
| 19 | 0.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2442` |
| 20 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 21 | 0.50% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 22 | 0.50% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 23 | 0.47% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.41% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.41% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 28 | 0.33% | `net.minecraft.world.level.chunk.status.ChunkStatusTasks.lambda$buildTerrain$0(ChunkAccess) line: 130` |
| 29 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2421` |
| 30 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2422` |

GC pauses: 0, total 0 ms

