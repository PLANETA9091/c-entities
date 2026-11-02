# JVM hotspots (JFR ExecutionSample)

Samples: 4934

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9116` |
| 2 | 8.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5403` |
| 3 | 6.30% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 5 | 5.51% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3996` |
| 6 | 4.88% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9063` |
| 7 | 4.86% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3869` |
| 9 | 1.68% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3949` |
| 10 | 1.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6149` |
| 11 | 1.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.58% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 13 | 1.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 14 | 1.32% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 1.24% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9077` |
| 16 | 1.13% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 17 | 0.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5409` |
| 18 | 0.93% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 19 | 0.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5440` |
| 20 | 0.79% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8757` |
| 21 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 22 | 0.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 23 | 0.71% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.63% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9067` |
| 25 | 0.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6139` |
| 26 | 0.47% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8759` |
| 27 | 0.45% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 28 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5448` |
| 29 | 0.41% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1212` |
| 30 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5481` |
| 31 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9074` |
| 32 | 0.28% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 33 | 0.24% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 34 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 35 | 0.22% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 36 | 0.22% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3944` |
| 37 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5352` |
| 38 | 0.22% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 39 | 0.20% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 40 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2709` |
| 41 | 0.20% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 42 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5349` |
| 43 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3881` |
| 44 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 45 | 0.18% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 46 | 0.18% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 47 | 0.18% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 48 | 0.18% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5222` |
| 50 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3773` |
| 51 | 0.16% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 52 | 0.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 53 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5221` |
| 54 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5309` |
| 55 | 0.16% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 433` |
| 56 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3862` |
| 57 | 0.14% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 58 | 0.14% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 59 | 0.14% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2680` |
| 60 | 0.14% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.54% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.75% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 7.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9112` |
| 4 | 6.30% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 5 | 5.78% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3902` |
| 6 | 5.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5440` |
| 7 | 3.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6149` |
| 8 | 3.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 9 | 2.39% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 2.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9104` |
| 11 | 2.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3900` |
| 12 | 1.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3771` |
| 13 | 1.68% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.46% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9070` |
| 15 | 1.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 16 | 1.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2710` |
| 17 | 1.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 18 | 0.99% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9062` |
| 19 | 0.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3869` |
| 20 | 0.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 22 | 0.45% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity, Vec3) line: 1166` |
| 23 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9063` |
| 24 | 0.28% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.28% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 26 | 0.28% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 27 | 0.28% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 28 | 0.26% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 29 | 0.24% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 30 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3870` |

GC pauses: 0, total 0 ms

