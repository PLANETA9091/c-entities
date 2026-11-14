# JVM hotspots (JFR ExecutionSample)

Samples: 2759

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.18% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12263` |
| 2 | 3.08% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.01% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 4 | 2.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 5 | 2.79% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 6 | 2.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7760` |
| 9 | 1.70% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 10 | 1.59% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12282` |
| 11 | 1.45% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.23% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 15 | 1.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12487` |
| 16 | 0.94% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 17 | 0.83% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 18 | 0.80% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 19 | 0.76% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 20 | 0.69% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 21 | 0.65% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 22 | 0.65% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 23 | 0.62% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 24 | 0.54% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 25 | 0.54% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 26 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.47% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 28 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 29 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 30 | 0.40% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 31 | 0.40% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 32 | 0.40% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 33 | 0.36% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 34 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 35 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 36 | 0.33% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 37 | 0.33% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1910` |
| 38 | 0.29% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 39 | 0.29% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 40 | 0.29% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 41 | 0.29% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 42 | 0.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12300` |
| 43 | 0.25% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 44 | 0.25% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 76` |
| 45 | 0.25% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 46 | 0.25% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 47 | 0.25% | `net.minecraft.server.level.WorldGenRegion.getChunk(int, int) line: 175` |
| 48 | 0.25% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9356` |
| 49 | 0.25% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 50 | 0.22% | `java.lang.String.hashCode() line: 2433` |
| 51 | 0.22% | `java.util.ArrayList$Itr.next() line: 1058` |
| 52 | 0.22% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 30` |
| 53 | 0.22% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 54 | 0.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7786` |
| 55 | 0.22% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 56 | 0.22% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 57 | 0.22% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10652` |
| 58 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 59 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 60 | 0.18% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12487` |
| 2 | 3.59% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10602` |
| 3 | 3.19% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 5 | 2.57% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10669` |
| 6 | 2.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 7 | 2.14% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.07% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.70% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 10 | 1.63% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.63% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10622` |
| 12 | 1.45% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5377` |
| 13 | 1.41% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 14 | 1.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7786` |
| 15 | 1.05% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 16 | 1.01% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 17 | 0.83% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 18 | 0.80% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 19 | 0.76% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 20 | 0.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6450` |
| 21 | 0.65% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.65% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 23 | 0.62% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 25 | 0.54% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 26 | 0.51% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10630` |
| 27 | 0.47% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 28 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.has(Entity) line: 473` |
| 29 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12285` |
| 30 | 0.43% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |

GC pauses: 0, total 0 ms

