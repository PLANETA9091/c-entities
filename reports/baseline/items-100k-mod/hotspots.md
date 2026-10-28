# JVM hotspots (JFR ExecutionSample)

Samples: 6460

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 2 | 8.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4073` |
| 3 | 6.63% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.20% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2769` |
| 5 | 5.02% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 4.43% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.96% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 8 | 2.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 1.98% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 1.81% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2671` |
| 11 | 1.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 12 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4079` |
| 13 | 1.10% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 1.02% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 15 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.84% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 17 | 0.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 18 | 0.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4110` |
| 19 | 0.82% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2722` |
| 20 | 0.71% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 21 | 0.65% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7683` |
| 22 | 0.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1963` |
| 23 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7673` |
| 24 | 0.59% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7363` |
| 25 | 0.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4784` |
| 26 | 0.40% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 27 | 0.39% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7365` |
| 28 | 0.36% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 29 | 0.34% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 30 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4118` |
| 31 | 0.29% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 270` |
| 32 | 0.26% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 33 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 34 | 0.26% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 35 | 0.26% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 36 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2664` |
| 37 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 38 | 0.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 39 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3934` |
| 40 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 41 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2576` |
| 42 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4023` |
| 43 | 0.22% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 44 | 0.22% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 45 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 46 | 0.20% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 47 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 48 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 49 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1947` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4021` |
| 51 | 0.17% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 52 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 53 | 0.17% | `java.lang.Double.isFinite(double) line: 1023` |
| 54 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 55 | 0.15% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7652` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4022` |
| 57 | 0.15% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2724` |
| 59 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4072` |
| 60 | 0.14% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 433` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.42% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.91% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.64% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 5.65% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7718` |
| 5 | 5.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2692` |
| 6 | 5.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4110` |
| 7 | 3.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4784` |
| 8 | 2.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7710` |
| 9 | 2.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 2.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2574` |
| 11 | 1.97% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.18% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1486` |
| 14 | 1.18% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 15 | 1.15% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 16 | 1.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2691` |
| 17 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.94% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2671` |
| 19 | 0.88% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7676` |
| 20 | 0.70% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7668` |
| 21 | 0.68% | `dev.planeta9091.centities.CeHooks.isMergableMirror(ItemEntity) line: 358` |
| 22 | 0.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1948` |
| 23 | 0.39% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.37% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 26 | 0.36% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 27 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2672` |
| 28 | 0.34% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 29 | 0.29% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 375` |
| 30 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

GC pauses: 0, total 0 ms

