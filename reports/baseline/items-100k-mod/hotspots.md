# JVM hotspots (JFR ExecutionSample)

Samples: 6337

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.54% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9126` |
| 2 | 11.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5413` |
| 3 | 6.82% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 6 | 4.01% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4006` |
| 7 | 3.80% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9073` |
| 8 | 1.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6149` |
| 9 | 1.45% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 10 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.04% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 0.95% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9087` |
| 13 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3879` |
| 15 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.66% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 17 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 18 | 0.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6159` |
| 19 | 0.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5419` |
| 20 | 0.57% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 21 | 0.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 22 | 0.47% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3959` |
| 23 | 0.46% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.46% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8769` |
| 25 | 0.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5450` |
| 26 | 0.38% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 27 | 0.36% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 28 | 0.35% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 29 | 0.32% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5458` |
| 31 | 0.27% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 32 | 0.27% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9077` |
| 33 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5233` |
| 34 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 35 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 36 | 0.24% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 37 | 0.24% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8767` |
| 38 | 0.22% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 39 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2694` |
| 40 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2688` |
| 41 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5412` |
| 42 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2709` |
| 43 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5232` |
| 44 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3872` |
| 46 | 0.21% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9081` |
| 47 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5231` |
| 48 | 0.19% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.setBoolean(Object, boolean) line: 132` |
| 49 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3783` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2267` |
| 51 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 52 | 0.17% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3781` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3891` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3885` |
| 56 | 0.16% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 57 | 0.16% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 58 | 0.16% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.getTailOpaque() line: 80` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5318` |
| 60 | 0.16% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.73% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.72% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.85% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 5.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9122` |
| 5 | 5.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5450` |
| 6 | 4.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3912` |
| 7 | 4.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9114` |
| 8 | 2.73% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 9 | 2.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6159` |
| 10 | 2.35% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3781` |
| 13 | 1.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 0.85% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3910` |
| 15 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2710` |
| 16 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.74% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 18 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9080` |
| 19 | 0.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.43% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 21 | 0.41% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.36% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 23 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3879` |
| 24 | 0.36% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 25 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3880` |
| 26 | 0.32% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 27 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9072` |
| 28 | 0.28% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 29 | 0.25% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 762` |
| 30 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

