# JVM hotspots (JFR ExecutionSample)

Samples: 7481

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9193` |
| 2 | 11.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5416` |
| 3 | 7.75% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.60% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 5 | 4.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4009` |
| 6 | 4.42% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 4.25% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9107` |
| 8 | 1.95% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.27% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9152` |
| 11 | 0.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6152` |
| 12 | 0.84% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 0.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5453` |
| 16 | 0.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5422` |
| 17 | 0.65% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2738` |
| 18 | 0.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6162` |
| 19 | 0.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3882` |
| 20 | 0.53% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8772` |
| 21 | 0.52% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5461` |
| 23 | 0.45% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3962` |
| 24 | 0.43% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 25 | 0.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2700` |
| 26 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 27 | 0.33% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9114` |
| 28 | 0.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2691` |
| 29 | 0.29% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8770` |
| 30 | 0.28% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 31 | 0.27% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 32 | 0.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6153` |
| 33 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 34 | 0.21% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 35 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2690` |
| 36 | 0.21% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 37 | 0.20% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 38 | 0.20% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 39 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 40 | 0.19% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 41 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5234` |
| 42 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5365` |
| 43 | 0.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9189` |
| 44 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5235` |
| 45 | 0.16% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 46 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 47 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 48 | 0.16% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1215` |
| 49 | 0.16% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 50 | 0.15% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 51 | 0.15% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 52 | 0.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 53 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 55 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5236` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5363` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3875` |
| 58 | 0.15% | `java.lang.Double.isFinite(double) line: 1023` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3786` |
| 60 | 0.13% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.29% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 13.17% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 7.77% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 6.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9189` |
| 5 | 5.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5453` |
| 6 | 4.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3915` |
| 7 | 3.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9181` |
| 8 | 2.21% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2082` |
| 9 | 1.99% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3784` |
| 10 | 1.87% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6162` |
| 11 | 1.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 0.84% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 15 | 0.78% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3913` |
| 16 | 0.76% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9118` |
| 17 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.74% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2713` |
| 19 | 0.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.41% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 21 | 0.31% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.28% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 23 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3882` |
| 24 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 25 | 0.21% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.21% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 126` |
| 27 | 0.21% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.21% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 29 | 0.21% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9106` |
| 30 | 0.20% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |

GC pauses: 0, total 0 ms

