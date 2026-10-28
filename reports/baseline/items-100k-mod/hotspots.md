# JVM hotspots (JFR ExecutionSample)

Samples: 5773

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 2 | 10.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4073` |
| 3 | 5.65% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.21% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.94% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 4.04% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2769` |
| 7 | 2.86% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 8 | 1.82% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 10 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 0.83% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7683` |
| 13 | 0.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4784` |
| 14 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4110` |
| 16 | 0.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4079` |
| 17 | 0.68% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2671` |
| 18 | 0.62% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 19 | 0.61% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 20 | 0.59% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7365` |
| 21 | 0.57% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2722` |
| 22 | 0.55% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 23 | 0.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1963` |
| 24 | 0.50% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 25 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 26 | 0.36% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 270` |
| 27 | 0.35% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 28 | 0.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4118` |
| 29 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 30 | 0.31% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7363` |
| 31 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 32 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 33 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 34 | 0.24% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 35 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7673` |
| 36 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4022` |
| 37 | 0.23% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 38 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 39 | 0.23% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 40 | 0.21% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 41 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1944` |
| 42 | 0.21% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1947` |
| 44 | 0.21% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3933` |
| 46 | 0.19% | `java.lang.Double.isFinite(double) line: 1023` |
| 47 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4072` |
| 48 | 0.17% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 49 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 762` |
| 50 | 0.17% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7677` |
| 51 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4135` |
| 52 | 0.17% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 53 | 0.17% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 54 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4021` |
| 56 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3934` |
| 57 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2683` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2576` |
| 59 | 0.14% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 60 | 0.14% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.33% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 13.16% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.68% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 5.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4110` |
| 5 | 4.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7710` |
| 6 | 4.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7718` |
| 7 | 4.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2692` |
| 8 | 2.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.94% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2574` |
| 10 | 1.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4784` |
| 12 | 1.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.11% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1486` |
| 14 | 1.06% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 15 | 0.95% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2691` |
| 16 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2671` |
| 18 | 0.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7676` |
| 20 | 0.43% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.43% | `dev.planeta9091.centities.CeHooks.isMergableMirror(ItemEntity) line: 358` |
| 22 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1948` |
| 23 | 0.36% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.36% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 375` |
| 25 | 0.35% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 26 | 0.29% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.28% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 30` |
| 28 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 29 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.23% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

