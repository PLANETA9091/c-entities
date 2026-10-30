# JVM hotspots (JFR ExecutionSample)

Samples: 4746

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4936` |
| 2 | 10.07% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8691` |
| 3 | 4.70% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.57% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8638` |
| 5 | 4.45% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3637` |
| 6 | 3.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3529` |
| 7 | 2.65% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 8 | 2.36% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.19% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3590` |
| 10 | 2.15% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 11 | 1.77% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2598` |
| 12 | 1.62% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5729` |
| 15 | 1.60% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 16 | 1.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5727` |
| 17 | 1.05% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 18 | 0.97% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 0.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5000` |
| 20 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 22 | 0.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5037` |
| 23 | 0.55% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8332` |
| 24 | 0.53% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 25 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8652` |
| 26 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8642` |
| 27 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5078` |
| 28 | 0.42% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 29 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5045` |
| 30 | 0.34% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 31 | 0.32% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 32 | 0.32% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1121` |
| 33 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 34 | 0.29% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8649` |
| 35 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.27% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 37 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 38 | 0.23% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 39 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3522` |
| 40 | 0.23% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3585` |
| 41 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 42 | 0.21% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 43 | 0.21% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2610` |
| 45 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 46 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 47 | 0.19% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3592` |
| 49 | 0.19% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 508` |
| 50 | 0.19% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 51 | 0.17% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 52 | 0.15% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 53 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5006` |
| 55 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3535` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4819` |
| 57 | 0.15% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 58 | 0.13% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 59 | 0.13% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 60 | 0.13% | `java.lang.Integer.valueOf(int) line: 1006` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.02% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 10.28% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8687` |
| 4 | 4.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3550` |
| 5 | 4.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5729` |
| 6 | 3.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3549` |
| 8 | 2.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3432` |
| 9 | 2.42% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 1985` |
| 10 | 2.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8679` |
| 11 | 2.15% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 12 | 1.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5037` |
| 13 | 1.71% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2594` |
| 14 | 1.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.60% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8645` |
| 16 | 1.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3529` |
| 17 | 1.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 18 | 1.03% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2170` |
| 19 | 0.99% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.93% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2611` |
| 21 | 0.84% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8637` |
| 22 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 23 | 0.59% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 24 | 0.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3530` |
| 25 | 0.42% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 26 | 0.38% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8638` |
| 28 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.34% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 30 | 0.32% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |

GC pauses: 0, total 0 ms

