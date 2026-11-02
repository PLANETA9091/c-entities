# JVM hotspots (JFR ExecutionSample)

Samples: 5983

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9207` |
| 2 | 8.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5427` |
| 3 | 5.57% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 4 | 4.80% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 4.25% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 4.08% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9121` |
| 7 | 3.76% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4020` |
| 8 | 2.66% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 1.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3893` |
| 10 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5433` |
| 12 | 0.97% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 0.92% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.84% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3973` |
| 16 | 0.82% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9166` |
| 17 | 0.79% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2742` |
| 18 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6166` |
| 20 | 0.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6176` |
| 21 | 0.64% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2704` |
| 23 | 0.57% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8786` |
| 24 | 0.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5465` |
| 25 | 0.40% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8784` |
| 26 | 0.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5475` |
| 27 | 0.38% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 28 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3797` |
| 29 | 0.33% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 30 | 0.32% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 31 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 32 | 0.30% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 33 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9128` |
| 34 | 0.27% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 35 | 0.27% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 36 | 0.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 37 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 38 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 39 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2694` |
| 40 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2716` |
| 41 | 0.23% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9133` |
| 42 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2701` |
| 43 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 44 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 45 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5246` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3886` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9146` |
| 48 | 0.18% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 49 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 50 | 0.18% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 51 | 0.18% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2695` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5373` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9088` |
| 55 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 56 | 0.17% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 123` |
| 57 | 0.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 58 | 0.17% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 59 | 0.17% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 60 | 0.17% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.17% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.72% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9203` |
| 4 | 5.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5465` |
| 5 | 4.83% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 6 | 3.96% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3926` |
| 7 | 3.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6176` |
| 8 | 2.82% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9195` |
| 9 | 2.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3795` |
| 10 | 2.57% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2086` |
| 11 | 1.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.62% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3924` |
| 15 | 0.99% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 16 | 0.85% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3893` |
| 17 | 0.79% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2717` |
| 18 | 0.79% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9132` |
| 19 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 20 | 0.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9120` |
| 22 | 0.42% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.40% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 24 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3894` |
| 25 | 0.33% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.33% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 27 | 0.33% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 28 | 0.30% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 29 | 0.28% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 30 | 0.27% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |

GC pauses: 0, total 0 ms

