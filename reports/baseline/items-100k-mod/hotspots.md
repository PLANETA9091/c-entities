# JVM hotspots (JFR ExecutionSample)

Samples: 4852

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 7.61% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 3 | 5.42% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 5 | 4.49% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 6 | 4.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.34% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 8 | 2.76% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 10 | 1.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.50% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 13 | 0.99% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2764` |
| 14 | 0.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4121` |
| 15 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 0.85% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 17 | 0.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.82% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 0.78% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 20 | 0.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 21 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7724` |
| 22 | 0.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 23 | 0.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 24 | 0.47% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 25 | 0.45% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7414` |
| 26 | 0.45% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 27 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 28 | 0.39% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2760` |
| 29 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4160` |
| 30 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 31 | 0.33% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7416` |
| 32 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 33 | 0.27% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 34 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 35 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2618` |
| 36 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 37 | 0.25% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 38 | 0.25% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 39 | 0.25% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7703` |
| 40 | 0.25% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7731` |
| 41 | 0.25% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 42 | 0.23% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 43 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3976` |
| 44 | 0.21% | `java.util.Collection.stream() line: 748` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2706` |
| 46 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2725` |
| 47 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 49 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4114` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 51 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4061` |
| 52 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 53 | 0.16% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 54 | 0.16% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 55 | 0.16% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 56 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4064` |
| 57 | 0.16% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2766` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4065` |
| 59 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 60 | 0.14% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.98% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 12.18% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 6.10% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 4 | 5.69% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 5 | 5.44% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 6 | 3.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2734` |
| 7 | 3.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 8 | 2.93% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 9 | 2.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2616` |
| 10 | 2.12% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.81% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2733` |
| 13 | 1.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 15 | 1.11% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 16 | 1.05% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7727` |
| 17 | 0.91% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 18 | 0.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 21 | 0.56% | `dev.planeta9091.centities.CeHooks.isMergableMirror(ItemEntity) line: 400` |
| 22 | 0.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2714` |
| 23 | 0.45% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 417` |
| 24 | 0.41% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7719` |
| 26 | 0.37% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.31% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 28 | 0.31% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 29 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 30 | 0.27% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |

GC pauses: 0, total 0 ms

