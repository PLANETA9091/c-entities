# JVM hotspots (JFR ExecutionSample)

Samples: 2319

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.51% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 5.09% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12967` |
| 3 | 2.63% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 2.41% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 6 | 2.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.72% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 8 | 1.55% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.47% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 10 | 1.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8299` |
| 11 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.25% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12986` |
| 15 | 0.99% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 16 | 0.95% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 17 | 0.91% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 18 | 0.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13209` |
| 19 | 0.82% | `java.util.HashMap.getNode(Object) line: 586` |
| 20 | 0.82% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 21 | 0.65% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 22 | 0.60% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 23 | 0.60% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 24 | 0.56% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 25 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 26 | 0.52% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 27 | 0.52% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.52% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 29 | 0.47% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 30 | 0.47% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 31 | 0.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13105` |
| 32 | 0.47% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 33 | 0.43% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 34 | 0.43% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 35 | 0.43% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11191` |
| 36 | 0.43% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 37 | 0.39% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 38 | 0.34% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 39 | 0.34% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 40 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 41 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 42 | 0.34% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 43 | 0.34% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1927` |
| 44 | 0.34% | `java.util.ArrayList.isEmpty() line: 263` |
| 45 | 0.34% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 46 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 47 | 0.30% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 48 | 0.30% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 49 | 0.30% | `com.mojang.serialization.DataResult.success(Object, Lifecycle) line: 41` |
| 50 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 51 | 0.30% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 52 | 0.30% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 53 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13004` |
| 54 | 0.30% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1933` |
| 55 | 0.30% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 56 | 0.26% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 57 | 0.26% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 58 | 0.26% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2060` |
| 59 | 0.26% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 60 | 0.26% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5188` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13209` |
| 2 | 6.34% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11208` |
| 3 | 3.88% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.36% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11141` |
| 5 | 2.72% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 2.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 7 | 2.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.72% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 9 | 1.60% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 10 | 1.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8325` |
| 11 | 1.47% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.42% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 13 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 1.21% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 15 | 1.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6858` |
| 16 | 0.99% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5771` |
| 17 | 0.99% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 18 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 19 | 0.82% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 20 | 0.73% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 21 | 0.65% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.65% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.65% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.65% | `java.util.HashMap.containsKey(Object) line: 603` |
| 25 | 0.65% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12992` |
| 26 | 0.60% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 27 | 0.60% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11171` |
| 28 | 0.52% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 29 | 0.52% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11161` |
| 30 | 0.47% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

