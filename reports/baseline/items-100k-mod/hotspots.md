# JVM hotspots (JFR ExecutionSample)

Samples: 4287

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7511` |
| 2 | 9.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4571` |
| 3 | 6.53% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7458` |
| 5 | 2.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 6 | 2.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3862` |
| 7 | 2.01% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 8 | 2.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2430` |
| 9 | 1.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.31% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 11 | 1.17% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 12 | 1.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 0.98% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7472` |
| 14 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1764` |
| 15 | 0.75% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 16 | 0.58% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2568` |
| 17 | 0.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2384` |
| 18 | 0.44% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 19 | 0.44% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 20 | 0.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2459` |
| 21 | 0.42% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 22 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 23 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7462` |
| 24 | 0.37% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 25 | 0.37% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 26 | 0.35% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 27 | 0.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1748` |
| 28 | 0.33% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 29 | 0.30% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 30 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3899` |
| 31 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2375` |
| 32 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 33 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 34 | 0.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4573` |
| 35 | 0.23% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 36 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3907` |
| 37 | 0.23% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 38 | 0.21% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 39 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 40 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 41 | 0.21% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 42 | 0.21% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 43 | 0.21% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.21% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7154` |
| 46 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 47 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7466` |
| 49 | 0.16% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 50 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 51 | 0.16% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 52 | 0.16% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 53 | 0.16% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 54 | 0.16% | `net.minecraft.server.level.WorldGenRegion.getChunk(int, int) line: 175` |
| 55 | 0.16% | `java.util.Collection.stream() line: 748` |
| 56 | 0.16% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 57 | 0.16% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3868` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2373` |
| 60 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.42% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 10.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 6.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7507` |
| 4 | 5.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7499` |
| 5 | 4.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4573` |
| 6 | 3.24% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 8 | 2.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3899` |
| 9 | 2.15% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.52% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1316` |
| 11 | 1.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2373` |
| 12 | 1.19% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 13 | 1.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 0.63% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 15 | 0.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2491` |
| 16 | 0.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1749` |
| 17 | 0.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.40% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.40% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 21 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7465` |
| 22 | 0.37% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 23 | 0.37% | `net.minecraft.util.StaticCache2D.lambda$map$0(StaticCache2D$MappingFunction, int, int) line: 36` |
| 24 | 0.37% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 25 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 26 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2453` |
| 27 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.28% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 30 | 0.28% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7457` |

GC pauses: 0, total 0 ms

