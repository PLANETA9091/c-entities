# JVM hotspots (JFR ExecutionSample)

Samples: 5241

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 2 | 8.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4073` |
| 3 | 6.05% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.50% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 4.10% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2769` |
| 7 | 2.92% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 8 | 2.12% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.11% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7683` |
| 10 | 1.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 0.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4784` |
| 13 | 0.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 14 | 0.84% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 15 | 0.82% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2671` |
| 16 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.73% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 18 | 0.73% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 0.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4079` |
| 20 | 0.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4110` |
| 21 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 22 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4118` |
| 23 | 0.42% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7363` |
| 24 | 0.42% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2722` |
| 25 | 0.32% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 26 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2677` |
| 27 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 28 | 0.31% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 29 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 30 | 0.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1963` |
| 31 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2664` |
| 32 | 0.29% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7673` |
| 33 | 0.29% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 34 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4135` |
| 35 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 36 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4022` |
| 37 | 0.27% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 270` |
| 38 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 39 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3934` |
| 40 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 41 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4023` |
| 42 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7652` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7365` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2724` |
| 46 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 47 | 0.19% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 48 | 0.19% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 49 | 0.17% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState, boolean) line: 144` |
| 50 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 51 | 0.15% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 52 | 0.15% | `java.math.MutableBigInteger.mulsub(int[], int[], int, int, int) line: 675` |
| 53 | 0.15% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 54 | 0.15% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 55 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.task.ChunkLoadTask.<init>(ChunkTaskScheduler, ServerLevel, int, int, NewChunkHolder, Priority) line: 49` |
| 56 | 0.15% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2718` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2766` |
| 59 | 0.15% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4121` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.99% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.65% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.07% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4110` |
| 5 | 4.75% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7718` |
| 6 | 4.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2692` |
| 7 | 3.84% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7710` |
| 8 | 2.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4784` |
| 9 | 2.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2574` |
| 10 | 1.98% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 0.95% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 0.92% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2691` |
| 14 | 0.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 15 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.73% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1486` |
| 17 | 0.73% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 18 | 0.67% | `dev.planeta9091.centities.CeHooks.isMergableMirror(ItemEntity) line: 358` |
| 19 | 0.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2671` |
| 20 | 0.48% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7676` |
| 22 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 23 | 0.34% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.34% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 25 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2672` |
| 26 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1948` |
| 28 | 0.27% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 375` |
| 29 | 0.23% | `java.math.BigInteger.multiplyToLen(int[], int, int[], int, int[]) line: 1844` |
| 30 | 0.23% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |

GC pauses: 0, total 0 ms

