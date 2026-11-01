# JVM hotspots (JFR ExecutionSample)

Samples: 5505

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.12% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 2 | 6.98% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 3 | 5.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 4 | 5.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5145` |
| 5 | 4.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5874` |
| 6 | 3.71% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.58% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 8 | 1.98% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3770` |
| 9 | 1.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 10 | 1.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3643` |
| 11 | 1.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.58% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.56% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 14 | 1.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 1.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 0.96% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 17 | 0.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5151` |
| 18 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 19 | 0.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5872` |
| 20 | 0.85% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3723` |
| 21 | 0.82% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 22 | 0.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5182` |
| 23 | 0.76% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8477` |
| 24 | 0.58% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 25 | 0.58% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8479` |
| 26 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8787` |
| 27 | 0.51% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 28 | 0.36% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 29 | 0.36% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 30 | 0.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5223` |
| 31 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3636` |
| 33 | 0.29% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 34 | 0.29% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 35 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 36 | 0.27% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 37 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3655` |
| 38 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 39 | 0.24% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3767` |
| 40 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 41 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 42 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2688` |
| 43 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2709` |
| 44 | 0.22% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 45 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3547` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4964` |
| 48 | 0.20% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 49 | 0.18% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 50 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 51 | 0.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 52 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3545` |
| 53 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5144` |
| 54 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4963` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3725` |
| 56 | 0.16% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3718` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2776` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5190` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3676` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3540` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.10% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8832` |
| 2 | 9.77% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5182` |
| 4 | 5.85% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 5.27% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 3.98% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 7 | 3.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5874` |
| 8 | 2.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3545` |
| 9 | 2.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3676` |
| 10 | 1.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8824` |
| 11 | 1.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.67% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.56% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 14 | 1.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3643` |
| 15 | 1.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 16 | 1.34% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8790` |
| 17 | 1.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3674` |
| 18 | 1.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2710` |
| 20 | 0.82% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3644` |
| 21 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 22 | 0.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 23 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8782` |
| 24 | 0.44% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.42% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.36% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 27 | 0.35% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.35% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 29 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 30 | 0.27% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |

GC pauses: 0, total 0 ms

