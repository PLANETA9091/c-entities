# JVM hotspots (JFR ExecutionSample)

Samples: 5966

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8068` |
| 2 | 9.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4386` |
| 3 | 6.55% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.18% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.61% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.62% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8015` |
| 7 | 3.27% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3081` |
| 8 | 2.43% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.17% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 11 | 1.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5104` |
| 12 | 0.94% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8029` |
| 13 | 0.92% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2973` |
| 14 | 0.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.79% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2213` |
| 17 | 0.74% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2194` |
| 18 | 0.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4423` |
| 19 | 0.64% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1872` |
| 20 | 0.62% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 21 | 0.62% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4392` |
| 23 | 0.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5106` |
| 24 | 0.44% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3034` |
| 25 | 0.42% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 26 | 0.39% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 27 | 0.39% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7711` |
| 28 | 0.37% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 29 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 30 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2966` |
| 31 | 0.32% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7709` |
| 32 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 33 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4431` |
| 34 | 0.28% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8019` |
| 35 | 0.27% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 276` |
| 36 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 37 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 38 | 0.25% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 39 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 40 | 0.23% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 41 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2189` |
| 42 | 0.23% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 43 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 44 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 45 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 46 | 0.18% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 47 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 48 | 0.18% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 49 | 0.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 50 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2191` |
| 51 | 0.18% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4455` |
| 53 | 0.17% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 54 | 0.17% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4247` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2871` |
| 57 | 0.15% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 58 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 59 | 0.15% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 60 | 0.15% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.96% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.77% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.55% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 5.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8064` |
| 5 | 4.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4423` |
| 6 | 4.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8056` |
| 7 | 3.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2994` |
| 8 | 2.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2876` |
| 9 | 2.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1713` |
| 10 | 2.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5106` |
| 11 | 1.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.44% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 0.87% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 15 | 0.85% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 552` |
| 16 | 0.82% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2198` |
| 17 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2993` |
| 19 | 0.67% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.55% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2973` |
| 21 | 0.52% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 22 | 0.49% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 23 | 0.42% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8022` |
| 25 | 0.39% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8015` |
| 27 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8014` |
| 28 | 0.28% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 530` |
| 29 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 30 | 0.25% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1846` |

GC pauses: 0, total 0 ms

