# JVM hotspots (JFR ExecutionSample)

Samples: 4414

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 8.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 3 | 6.28% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 5.85% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 5 | 4.89% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 6 | 4.30% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 7 | 4.26% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.70% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 10 | 1.81% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2764` |
| 11 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 13 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4121` |
| 14 | 1.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 15 | 1.13% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 16 | 1.13% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.91% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7414` |
| 18 | 0.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 19 | 0.86% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 20 | 0.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 22 | 0.72% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7724` |
| 23 | 0.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 24 | 0.61% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7416` |
| 25 | 0.59% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 26 | 0.59% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 27 | 0.50% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 28 | 0.48% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 29 | 0.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4160` |
| 30 | 0.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1678` |
| 31 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 32 | 0.36% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 33 | 0.34% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 761` |
| 34 | 0.32% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 35 | 0.32% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 36 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4184` |
| 37 | 0.27% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 38 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 39 | 0.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 40 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 41 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 42 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 43 | 0.20% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 44 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4114` |
| 45 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2725` |
| 46 | 0.20% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 47 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 48 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7731` |
| 50 | 0.18% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2760` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2618` |
| 52 | 0.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 53 | 0.16% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 54 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 55 | 0.16% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 56 | 0.16% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 57 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7728` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3976` |
| 60 | 0.16% | `java.lang.Iterable.forEach(Consumer) line: 75` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.07% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 4 | 6.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 5 | 5.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2734` |
| 6 | 4.30% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 7 | 3.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 8 | 3.06% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 2.81% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2616` |
| 10 | 2.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2733` |
| 11 | 1.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 12 | 1.88% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 13 | 1.81% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.56% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7727` |
| 15 | 1.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 16 | 1.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 17 | 1.16% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 19 | 0.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 20 | 0.77% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7719` |
| 21 | 0.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2714` |
| 22 | 0.57% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 430` |
| 23 | 0.50% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 24 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 25 | 0.36% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 26 | 0.36% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 417` |
| 27 | 0.29% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 28 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.27% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 30 | 0.27% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |

GC pauses: 0, total 0 ms

