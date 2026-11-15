# JVM hotspots (JFR ExecutionSample)

Samples: 2590

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.83% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12953` |
| 2 | 3.20% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.74% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 2.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 5 | 2.20% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 2.01% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13195` |
| 8 | 1.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.78% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.74% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 11 | 1.66% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12972` |
| 12 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.39% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 14 | 1.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8285` |
| 15 | 1.24% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 16 | 1.08% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 1.00% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 18 | 0.89% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 19 | 0.73% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 20 | 0.69% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 21 | 0.69% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 22 | 0.58% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 23 | 0.46% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 24 | 0.46% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 25 | 0.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.42% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 27 | 0.42% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 28 | 0.39% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 29 | 0.35% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 30 | 0.35% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 31 | 0.35% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 32 | 0.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13091` |
| 33 | 0.31% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 34 | 0.31% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 35 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 36 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12990` |
| 37 | 0.31% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 38 | 0.31% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9881` |
| 39 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 40 | 0.31% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 41 | 0.31% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 42 | 0.31% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 43 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 44 | 0.27% | `java.util.HashMap.getNode(Object) line: 588` |
| 45 | 0.27% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 46 | 0.27% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 47 | 0.27% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 8127` |
| 48 | 0.27% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 49 | 0.27% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2060` |
| 50 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 51 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 52 | 0.23% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11177` |
| 53 | 0.23% | `net.minecraft.world.entity.EntityDimensions.makeBoundingBox(double, double, double) line: 22` |
| 54 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 55 | 0.19% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 56 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 57 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 954` |
| 58 | 0.19% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 59 | 0.19% | `java.util.Objects.hashCode(Object) line: 97` |
| 60 | 0.19% | `java.util.HashMap.put(Object, Object) line: 619` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13195` |
| 2 | 3.44% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.97% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11194` |
| 4 | 2.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 5 | 2.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 6 | 2.43% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11127` |
| 7 | 2.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 2.20% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 9 | 1.78% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.74% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11147` |
| 11 | 1.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.39% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 13 | 1.35% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 14 | 1.08% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 15 | 1.08% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8311` |
| 16 | 1.04% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11155` |
| 17 | 1.00% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5757` |
| 18 | 0.89% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.89% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 20 | 0.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6844` |
| 21 | 0.73% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 22 | 0.69% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 23 | 0.66% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.66% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 25 | 0.62% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.62% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 28 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 29 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 30 | 0.39% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |

GC pauses: 0, total 0 ms

