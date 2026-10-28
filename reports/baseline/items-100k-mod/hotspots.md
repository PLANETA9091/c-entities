# JVM hotspots (JFR ExecutionSample)

Samples: 5143

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 8.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 3 | 7.37% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.79% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 5 | 4.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 4.24% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.66% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 8 | 2.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 2.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 10 | 1.75% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 11 | 1.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 12 | 1.21% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.13% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 14 | 1.05% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 0.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4121` |
| 16 | 0.95% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2764` |
| 17 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.68% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 0.64% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7724` |
| 20 | 0.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 21 | 0.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 22 | 0.56% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7414` |
| 23 | 0.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 24 | 0.52% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 25 | 0.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 26 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4160` |
| 27 | 0.37% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 28 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 29 | 0.33% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7416` |
| 30 | 0.33% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 31 | 0.29% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 32 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 33 | 0.27% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 34 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 35 | 0.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 36 | 0.27% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2760` |
| 37 | 0.27% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 38 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4065` |
| 39 | 0.25% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 40 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 41 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 42 | 0.23% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2808` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3976` |
| 45 | 0.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 46 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2618` |
| 47 | 0.17% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7703` |
| 48 | 0.16% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 49 | 0.16% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 50 | 0.16% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 51 | 0.16% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 52 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4066` |
| 53 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2706` |
| 54 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7731` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4064` |
| 56 | 0.14% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 57 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 58 | 0.14% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 59 | 0.14% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 60 | 0.14% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 761` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.52% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.41% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 7.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 4 | 7.41% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 5 | 4.90% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 6 | 3.99% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2734` |
| 7 | 2.96% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 2.80% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 9 | 2.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 10 | 2.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2616` |
| 12 | 1.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2733` |
| 13 | 1.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 15 | 1.05% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 16 | 1.05% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 17 | 0.95% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 18 | 0.95% | `dev.planeta9091.centities.CeHooks.isMergableMirror(ItemEntity) line: 400` |
| 19 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 20 | 0.80% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7727` |
| 21 | 0.70% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 22 | 0.52% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7719` |
| 23 | 0.35% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.35% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 25 | 0.29% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.29% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 28 | 0.29% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 29 | 0.27% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 30 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2714` |

GC pauses: 0, total 0 ms

