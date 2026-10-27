# JVM hotspots (JFR ExecutionSample)

Samples: 3836

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7216` |
| 2 | 5.21% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 4.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4276` |
| 4 | 3.44% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7163` |
| 5 | 3.36% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 2.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 7 | 2.03% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 8 | 1.96% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2089` |
| 9 | 1.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3567` |
| 10 | 1.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2135` |
| 12 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.70% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 15 | 0.70% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7177` |
| 16 | 0.63% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.60% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 18 | 0.55% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 19 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 20 | 0.50% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1484` |
| 21 | 0.50% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2273` |
| 22 | 0.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1468` |
| 23 | 0.42% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 24 | 0.36% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 25 | 0.36% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 26 | 0.34% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 27 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 28 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7167` |
| 29 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3573` |
| 30 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3604` |
| 31 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2175` |
| 32 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 33 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 34 | 0.29% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 35 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2073` |
| 36 | 0.26% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 37 | 0.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4278` |
| 38 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2164` |
| 39 | 0.23% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1873` |
| 40 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1465` |
| 41 | 0.23% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 42 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2092` |
| 43 | 0.21% | `jdk.internal.foreign.HeapMemorySegmentImpl$OfByte.unsafeGetBase() line: 114` |
| 44 | 0.21% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 45 | 0.18% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 46 | 0.18% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState, boolean) line: 144` |
| 47 | 0.18% | `com.mojang.serialization.Encoder$1.encode(Object, DynamicOps, Object) line: 25` |
| 48 | 0.18% | `java.lang.Class.cast(Object) line: 3491` |
| 49 | 0.18% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 51 | 0.18% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 52 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2121` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2122` |
| 55 | 0.16% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 56 | 0.16% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 57 | 0.16% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 740` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 258` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3612` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 6859` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.09% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 6.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4278` |
| 3 | 5.29% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 4.80% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7212` |
| 5 | 4.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7204` |
| 6 | 3.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2078` |
| 7 | 3.55% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 2.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3604` |
| 10 | 1.82% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2089` |
| 11 | 1.43% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1081` |
| 12 | 1.04% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 0.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 15 | 0.60% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 16 | 0.57% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2196` |
| 17 | 0.55% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 18 | 0.52% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 0.52% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 20 | 0.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1469` |
| 21 | 0.52% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 22 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2090` |
| 23 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7170` |
| 24 | 0.39% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.36% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 26 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 28 | 0.31% | `java.util.zip.ZipFile.getEntry(String) line: 294` |
| 29 | 0.31% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 30 | 0.29% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |

GC pauses: 0, total 0 ms

