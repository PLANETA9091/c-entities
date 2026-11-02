# JVM hotspots (JFR ExecutionSample)

Samples: 6268

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9207` |
| 2 | 9.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5427` |
| 3 | 7.15% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.80% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 5 | 3.99% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4020` |
| 6 | 3.67% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.32% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9121` |
| 8 | 1.18% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.12% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 11 | 1.07% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9166` |
| 12 | 1.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6166` |
| 13 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3893` |
| 16 | 0.80% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2742` |
| 17 | 0.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5465` |
| 18 | 0.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2704` |
| 19 | 0.61% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5433` |
| 20 | 0.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6176` |
| 21 | 0.43% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.38% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3973` |
| 23 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9128` |
| 24 | 0.37% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8786` |
| 25 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 26 | 0.30% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 27 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5475` |
| 28 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5246` |
| 29 | 0.29% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 30 | 0.29% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 31 | 0.29% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8784` |
| 32 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 33 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3795` |
| 34 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3797` |
| 35 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 36 | 0.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2694` |
| 37 | 0.26% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 38 | 0.26% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 39 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 40 | 0.24% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 41 | 0.21% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 42 | 0.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9203` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5376` |
| 44 | 0.19% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 45 | 0.19% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 46 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 47 | 0.19% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2695` |
| 49 | 0.19% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 50 | 0.19% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 51 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3886` |
| 52 | 0.19% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2274` |
| 53 | 0.18% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 54 | 0.18% | `java.lang.Double.isFinite(double) line: 1023` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 393` |
| 56 | 0.16% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 57 | 0.16% | `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock.unlock(ReentrantAreaLock$Node) line: 266` |
| 58 | 0.16% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 59 | 0.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9133` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.32% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 14.37% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.15% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 5.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9203` |
| 5 | 5.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5465` |
| 6 | 4.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3926` |
| 7 | 2.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9195` |
| 8 | 2.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2086` |
| 9 | 2.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6176` |
| 10 | 1.88% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3795` |
| 13 | 1.15% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 0.91% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2717` |
| 16 | 0.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3924` |
| 17 | 0.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 18 | 0.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9132` |
| 20 | 0.49% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 21 | 0.40% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.38% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.35% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 24 | 0.34% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 762` |
| 25 | 0.30% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 26 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3893` |
| 27 | 0.26% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 28 | 0.26% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 29 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3894` |
| 30 | 0.26% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |

GC pauses: 0, total 0 ms

