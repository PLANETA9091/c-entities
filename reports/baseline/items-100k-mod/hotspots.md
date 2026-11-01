# JVM hotspots (JFR ExecutionSample)

Samples: 4610

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5207` |
| 2 | 9.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8920` |
| 3 | 6.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 4 | 6.16% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3800` |
| 5 | 4.90% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8867` |
| 6 | 4.71% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 7 | 4.32% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3673` |
| 9 | 2.45% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 10 | 1.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5943` |
| 11 | 1.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.48% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3753` |
| 13 | 1.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 14 | 1.19% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5213` |
| 16 | 1.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5953` |
| 17 | 1.08% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 18 | 0.95% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8881` |
| 19 | 0.91% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 20 | 0.89% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 21 | 0.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 22 | 0.74% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5244` |
| 23 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 24 | 0.56% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8871` |
| 25 | 0.52% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8561` |
| 26 | 0.50% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 27 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 28 | 0.41% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 29 | 0.39% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8563` |
| 30 | 0.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5285` |
| 31 | 0.35% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1212` |
| 32 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 33 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 34 | 0.28% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8878` |
| 35 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5252` |
| 36 | 0.24% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 37 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 38 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 39 | 0.22% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 40 | 0.22% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 41 | 0.22% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 42 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 43 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5206` |
| 44 | 0.20% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 45 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2688` |
| 46 | 0.17% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5192` |
| 47 | 0.17% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 48 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5218` |
| 49 | 0.15% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 50 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 51 | 0.15% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 52 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2694` |
| 53 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3679` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2709` |
| 55 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5154` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8875` |
| 57 | 0.13% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 135` |
| 58 | 0.13% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 59 | 0.13% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.getTailOpaque() line: 80` |
| 60 | 0.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5025` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.08% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.80% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 7.03% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8916` |
| 4 | 6.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5244` |
| 5 | 6.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3706` |
| 6 | 4.77% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 7 | 3.41% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 8 | 3.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5953` |
| 9 | 3.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 2.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3575` |
| 11 | 1.93% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8908` |
| 12 | 1.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3704` |
| 13 | 1.63% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8874` |
| 14 | 1.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 16 | 1.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3673` |
| 17 | 0.93% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 18 | 0.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2710` |
| 20 | 0.76% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8866` |
| 21 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 22 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8867` |
| 23 | 0.41% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.41% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 25 | 0.37% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity, Vec3) line: 1166` |
| 26 | 0.33% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 27 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.30% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 29 | 0.30% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 30 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.has(Entity) line: 473` |

GC pauses: 0, total 0 ms

