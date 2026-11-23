# JVM hotspots (JFR ExecutionSample)

Samples: 2319

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.79% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 4.53% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16008` |
| 3 | 3.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.37% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15128` |
| 5 | 2.20% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 1.90% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.68% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 8 | 1.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.38% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 10 | 1.29% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 11 | 1.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16288` |
| 12 | 1.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.21% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 1.21% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 15 | 1.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10644` |
| 16 | 1.03% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 17 | 1.03% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 18 | 0.95% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 19 | 0.78% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 20 | 0.78% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 22 | 0.65% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 23 | 0.56% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 24 | 0.52% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 25 | 0.52% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16029` |
| 26 | 0.52% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 27 | 0.47% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 28 | 0.47% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 29 | 0.47% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13750` |
| 30 | 0.47% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 31 | 0.43% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 32 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 33 | 0.43% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 34 | 0.43% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 35 | 0.43% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 36 | 0.43% | `java.util.HashMap.getNode(Object) line: 586` |
| 37 | 0.43% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 38 | 0.34% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 39 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 40 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 41 | 0.30% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 42 | 0.30% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 43 | 0.30% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 44 | 0.30% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 30` |
| 45 | 0.30% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1146` |
| 46 | 0.30% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 47 | 0.30% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 48 | 0.30% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2647` |
| 49 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 50 | 0.26% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 51 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 446` |
| 52 | 0.26% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2641` |
| 53 | 0.26% | `java.util.ArrayList$Itr.next() line: 1058` |
| 54 | 0.26% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5188` |
| 55 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 56 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 57 | 0.22% | `java.math.MutableBigInteger.mulsub(int[], int[], int, int, int) line: 675` |
| 58 | 0.22% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 59 | 0.22% | `com.mojang.serialization.Encoder$1.encode(Object, DynamicOps, Object) line: 25` |
| 60 | 0.22% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16288` |
| 2 | 4.48% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13767` |
| 3 | 4.14% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.75% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13700` |
| 5 | 2.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 6 | 2.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 7 | 2.20% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 8 | 1.90% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.64% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 10 | 1.55% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10739` |
| 12 | 1.29% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 1.21% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15136` |
| 15 | 1.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8396` |
| 16 | 1.21% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 17 | 1.16% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13720` |
| 18 | 1.03% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 19 | 0.95% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16014` |
| 20 | 0.82% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.has(Entity) line: 473` |
| 22 | 0.78% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 23 | 0.73% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.69% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 25 | 0.65% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.60% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 27 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 28 | 0.56% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 29 | 0.56% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 30 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms

