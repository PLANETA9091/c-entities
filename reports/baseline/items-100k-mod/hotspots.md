# JVM hotspots (JFR ExecutionSample)

Samples: 6000

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4962` |
| 2 | 11.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8717` |
| 3 | 5.05% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.20% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.58% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 6 | 3.12% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3663` |
| 7 | 3.12% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8664` |
| 8 | 3.10% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3537` |
| 10 | 1.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5026` |
| 11 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 0.98% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 13 | 0.92% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2601` |
| 14 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.80% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 16 | 0.73% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8678` |
| 17 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.65% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 19 | 0.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5063` |
| 20 | 0.62% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 21 | 0.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5755` |
| 22 | 0.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5753` |
| 23 | 0.55% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3616` |
| 24 | 0.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 25 | 0.35% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 26 | 0.33% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 27 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2613` |
| 28 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3442` |
| 29 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 30 | 0.27% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 31 | 0.27% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 32 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 33 | 0.27% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 34 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4932` |
| 35 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5071` |
| 36 | 0.23% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 37 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 38 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2598` |
| 39 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4975` |
| 40 | 0.22% | `java.util.HashMap.getNode(Object) line: 577` |
| 41 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3530` |
| 42 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4844` |
| 43 | 0.22% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 44 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 45 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 46 | 0.20% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.18% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 48 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2591` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8668` |
| 50 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4931` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5025` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3618` |
| 53 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 54 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 55 | 0.17% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2592` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2388` |
| 58 | 0.17% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 59 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4845` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3611` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.12% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.07% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8713` |
| 5 | 3.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8705` |
| 6 | 3.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3569` |
| 7 | 3.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3440` |
| 8 | 3.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5755` |
| 9 | 2.70% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2597` |
| 10 | 2.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 1988` |
| 11 | 1.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5063` |
| 13 | 1.23% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 14 | 1.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3568` |
| 15 | 1.12% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 0.97% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2173` |
| 17 | 0.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3537` |
| 18 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2614` |
| 21 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8671` |
| 22 | 0.43% | `java.util.HashMap.get(Object) line: 565` |
| 23 | 0.35% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.33% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 637` |
| 25 | 0.30% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 26 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 28 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3538` |
| 29 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.23% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |

GC pauses: 0, total 0 ms

