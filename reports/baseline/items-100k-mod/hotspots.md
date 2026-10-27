# JVM hotspots (JFR ExecutionSample)

Samples: 3781

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.70% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 2 | 5.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 3 | 4.55% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2492` |
| 4 | 3.23% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.62% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 2.49% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2417` |
| 8 | 1.80% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 9 | 1.59% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 10 | 1.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2463` |
| 11 | 1.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 12 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4606` |
| 15 | 1.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3895` |
| 16 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 17 | 0.85% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 18 | 0.71% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 19 | 0.53% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 20 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7495` |
| 21 | 0.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2401` |
| 22 | 0.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2449` |
| 23 | 0.48% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 24 | 0.45% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 25 | 0.45% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 26 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2503` |
| 27 | 0.45% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 28 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3901` |
| 29 | 0.37% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1458` |
| 30 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2427` |
| 31 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2421` |
| 32 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 33 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 34 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 35 | 0.32% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7187` |
| 36 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3932` |
| 37 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 38 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 39 | 0.26% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 40 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 41 | 0.26% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 42 | 0.24% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 43 | 0.24% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 99` |
| 44 | 0.24% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2601` |
| 45 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7474` |
| 46 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2450` |
| 47 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 48 | 0.21% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 49 | 0.21% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 50 | 0.21% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 51 | 0.21% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 52 | 0.21% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 53 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 54 | 0.19% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 55 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2482` |
| 56 | 0.19% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7505` |
| 57 | 0.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4607` |
| 58 | 0.16% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 59 | 0.16% | `net.minecraft.world.level.biome.Climate$RTree.lambda$comparator$0(int, boolean, Climate$RTree$Node) line: 405` |
| 60 | 0.16% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5195` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.94% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7540` |
| 2 | 12.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4606` |
| 3 | 5.66% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.80% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3932` |
| 6 | 2.67% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.57% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2406` |
| 8 | 2.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 1.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2417` |
| 10 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 11 | 1.32% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7498` |
| 12 | 1.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2418` |
| 13 | 1.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7532` |
| 14 | 1.22% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1320` |
| 15 | 1.01% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 0.90% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.71% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 18 | 0.63% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 19 | 0.61% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7490` |
| 20 | 0.56% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.50% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 22 | 0.45% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.45% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 24 | 0.45% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 322` |
| 25 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 26 | 0.42% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 27 | 0.37% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 28 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2524` |
| 29 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.util.ParallelSearchRadiusIteration.generateBFSOrder(int) line: 344` |
| 30 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2504` |

GC pauses: 0, total 0 ms

