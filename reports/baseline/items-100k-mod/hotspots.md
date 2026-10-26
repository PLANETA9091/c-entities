# JVM hotspots (JFR ExecutionSample)

Samples: 4339

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.93% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 9.59% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4002` |
| 3 | 4.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4198` |
| 4 | 3.60% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6444` |
| 5 | 2.58% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 6 | 2.44% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.14% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 8 | 1.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3489` |
| 9 | 1.73% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 10 | 1.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2076` |
| 11 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.31% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.99% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2030` |
| 16 | 0.71% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2214` |
| 17 | 0.67% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 18 | 0.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1426` |
| 19 | 0.53% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 20 | 0.51% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 21 | 0.51% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4042` |
| 22 | 0.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3526` |
| 23 | 0.41% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 24 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 25 | 0.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4200` |
| 26 | 0.39% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 27 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 28 | 0.35% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 29 | 0.32% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 30 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2062` |
| 31 | 0.28% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 32 | 0.28% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.setTailPlain(MultiThreadedQueue$LinkedNode) line: 66` |
| 33 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1407` |
| 34 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2105` |
| 35 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 36 | 0.25% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 37 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 38 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 39 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 40 | 0.21% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 41 | 0.21% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 42 | 0.21% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 43 | 0.18% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 44 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 45 | 0.18% | `java.lang.String.length() line: 1583` |
| 46 | 0.18% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 47 | 0.18% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.18% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4033` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3495` |
| 50 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2116` |
| 51 | 0.16% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 52 | 0.16% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 53 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2014` |
| 54 | 0.16% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3991` |
| 55 | 0.16% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 56 | 0.14% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 57 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 58 | 0.14% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 59 | 0.14% | `java.util.AbstractCollection.isEmpty() line: 91` |
| 60 | 0.14% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 765` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.82% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6444` |
| 2 | 10.60% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 5.30% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 5.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4200` |
| 5 | 3.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 6 | 3.57% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2019` |
| 8 | 2.35% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6445` |
| 9 | 2.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 2.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 11 | 1.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3526` |
| 12 | 1.45% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1060` |
| 13 | 1.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 0.94% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2030` |
| 16 | 0.81% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2137` |
| 17 | 0.71% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 18 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.65% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 20 | 0.60% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4002` |
| 21 | 0.53% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.51% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 23 | 0.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2031` |
| 24 | 0.41% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 25 | 0.41% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 26 | 0.37% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 28 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2116` |
| 29 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1411` |
| 30 | 0.28% | `net.minecraft.world.level.levelgen.synth.NoiseStack$SmearedPerlin.addToVolume(DensityBuffer, DensityVolume, double, double, float) line: 153` |

GC pauses: 0, total 0 ms

