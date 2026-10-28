# JVM hotspots (JFR ExecutionSample)

Samples: 5786

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 10.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 3 | 6.05% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.96% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 5 | 4.87% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 4.74% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 7 | 3.70% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 8 | 2.25% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 11 | 0.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 12 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 14 | 0.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 15 | 0.83% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 16 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4121` |
| 18 | 0.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 19 | 0.59% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 20 | 0.50% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7416` |
| 21 | 0.50% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 22 | 0.50% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 23 | 0.43% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2764` |
| 24 | 0.41% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 25 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4160` |
| 26 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 27 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7724` |
| 28 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 29 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 30 | 0.26% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 31 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 32 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 33 | 0.26% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 34 | 0.26% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7414` |
| 35 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3976` |
| 36 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 37 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2706` |
| 38 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4061` |
| 39 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 40 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4063` |
| 41 | 0.22% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 42 | 0.22% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2766` |
| 43 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 44 | 0.21% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.getTailOpaque() line: 80` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3975` |
| 46 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2618` |
| 47 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 48 | 0.19% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 49 | 0.19% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7703` |
| 51 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 52 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 53 | 0.17% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 54 | 0.17% | `java.lang.Double.isFinite(double) line: 1023` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2725` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7728` |
| 57 | 0.17% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 58 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 59 | 0.16% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 60 | 0.16% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.61% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.35% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 4 | 6.07% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 5 | 5.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 6 | 4.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 7 | 3.94% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2734` |
| 8 | 2.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 9 | 2.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2616` |
| 10 | 1.95% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.23% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.14% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 14 | 0.97% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 15 | 0.88% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2733` |
| 16 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 18 | 0.60% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7727` |
| 19 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.43% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.43% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 23 | 0.40% | `dev.planeta9091.centities.CeHooks.isMergableMirror(ItemEntity) line: 400` |
| 24 | 0.35% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 26 | 0.26% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 27 | 0.22% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 417` |
| 28 | 0.21% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.21% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 30 | 0.21% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.appendList(MultiThreadedQueue$LinkedNode, MultiThreadedQueue$LinkedNode) line: 983` |

GC pauses: 0, total 0 ms

