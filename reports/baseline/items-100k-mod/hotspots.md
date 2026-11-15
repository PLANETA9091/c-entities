# JVM hotspots (JFR ExecutionSample)

Samples: 6282

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7533` |
| 2 | 10.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8285` |
| 3 | 5.40% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 4.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7422` |
| 6 | 3.07% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 7 | 2.87% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.44% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 2.39% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12805` |
| 10 | 2.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13091` |
| 11 | 1.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5216` |
| 12 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 0.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.80% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 0.70% | `java.util.HashMap.getNode(Object) line: 586` |
| 16 | 0.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5220` |
| 17 | 0.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7378` |
| 18 | 0.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7354` |
| 19 | 0.43% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 20 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 21 | 0.38% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2464` |
| 22 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5222` |
| 23 | 0.32% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 24 | 0.32% | `java.util.ArrayList$Itr.next() line: 1058` |
| 25 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 26 | 0.29% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 27 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 28 | 0.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8295` |
| 29 | 0.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 30 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7217` |
| 31 | 0.22% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 33 | 0.22% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.22% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 35 | 0.19% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 36 | 0.19% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 37 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7643` |
| 38 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 39 | 0.18% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 40 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 41 | 0.18% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 42 | 0.16% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 43 | 0.16% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 44 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5224` |
| 45 | 0.14% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 46 | 0.14% | `net.minecraft.server.level.Ticket.moonrise$getRemoveDelay() line: 32` |
| 47 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5144` |
| 48 | 0.14% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12944` |
| 49 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5221` |
| 50 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5183` |
| 51 | 0.13% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 52 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 53 | 0.13% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 54 | 0.13% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 55 | 0.13% | `com.mojang.serialization.Encoder$1.encode(Object, DynamicOps, Object) line: 25` |
| 56 | 0.13% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 57 | 0.13% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 58 | 0.13% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5494` |
| 59 | 0.13% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 60 | 0.13% | `ru.planeta9091.cebaseline.BaselinePlugin.onTickEnd(ServerTickEndEvent) line: 565` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.90% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.00% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 5.40% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5757` |
| 4 | 4.12% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.96% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8295` |
| 6 | 3.17% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 3.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7364` |
| 8 | 2.56% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 9 | 2.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13182` |
| 10 | 1.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5145` |
| 11 | 1.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 13 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 0.67% | `java.util.HashMap.containsKey(Object) line: 603` |
| 15 | 0.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7360` |
| 16 | 0.41% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 17 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 18 | 0.35% | `net.minecraft.util.StaticCache2D.lambda$map$0(StaticCache2D$MappingFunction, int, int) line: 36` |
| 19 | 0.33% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 20 | 0.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13195` |
| 21 | 0.32% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 22 | 0.30% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 23 | 0.30% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 24 | 0.30% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 163` |
| 25 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 26 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.24% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.isJoinNonEmpty(VoxelShape, VoxelShape, BooleanOp) line: 1320` |
| 28 | 0.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7436` |
| 30 | 0.19% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.buildSurface(ChunkAccess, NoiseChunk, RandomState, BiomeManager, Set, MaterialRule, ServerLevel) line: 227` |

GC pauses: 0, total 0 ms

