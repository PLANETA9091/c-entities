# JVM hotspots (JFR ExecutionSample)

Samples: 4186

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.76% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 8.86% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4007` |
| 3 | 5.97% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 4 | 3.25% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 5 | 3.11% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6870` |
| 6 | 2.46% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.25% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 8 | 1.91% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 9 | 1.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3494` |
| 10 | 1.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2035` |
| 11 | 1.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2081` |
| 12 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 1.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.98% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.93% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 16 | 0.69% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 17 | 0.67% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1431` |
| 18 | 0.60% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 19 | 0.60% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 20 | 0.60% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 21 | 0.57% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2219` |
| 22 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 23 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2110` |
| 24 | 0.38% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 25 | 0.33% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 26 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 27 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 28 | 0.29% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 29 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 30 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 31 | 0.29% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4047` |
| 32 | 0.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 33 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2026` |
| 34 | 0.26% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 35 | 0.26% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 36 | 0.26% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 37 | 0.26% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue$LinkedNode.<init>(Object, MultiThreadedQueue$LinkedNode) line: 1250` |
| 38 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2121` |
| 39 | 0.24% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 40 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2019` |
| 41 | 0.24% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 42 | 0.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 43 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3531` |
| 44 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3500` |
| 45 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 954` |
| 46 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 47 | 0.17% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 48 | 0.17% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 49 | 0.17% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 50 | 0.17% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 51 | 0.17% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4038` |
| 52 | 0.17% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 54 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 55 | 0.14% | `net.minecraft.world.phys.shapes.DiscreteVoxelShape.forAllBoxes(DiscreteVoxelShape$IntLineConsumer, boolean) line: 284` |
| 56 | 0.14% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1863` |
| 57 | 0.14% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 58 | 0.14% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1415` |
| 59 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2106` |
| 60 | 0.12% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1873` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.40% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6870` |
| 2 | 9.53% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 6.45% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 5.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 5 | 3.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 6 | 3.25% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 7 | 3.06% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2024` |
| 9 | 2.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 2.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3531` |
| 11 | 2.15% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6871` |
| 12 | 1.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1065` |
| 13 | 1.19% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 0.98% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 0.88% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2035` |
| 16 | 0.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2142` |
| 17 | 0.69% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 18 | 0.62% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 0.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.60% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 21 | 0.50% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1416` |
| 23 | 0.38% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 24 | 0.36% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2036` |
| 26 | 0.31% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 27 | 0.29% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 28 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 29 | 0.26% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 30 | 0.26% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |

GC pauses: 0, total 0 ms

