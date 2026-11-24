# JVM hotspots (JFR ExecutionSample)

Samples: 2095

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.11% | `dev.planeta9091.centities.CeHooks.sitStepsEmpty(Object) line: 15343` |
| 2 | 4.96% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.10% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 5 | 1.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 6 | 1.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.58% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.38% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 10 | 1.38% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 11 | 1.34% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 1.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.24% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15210` |
| 14 | 1.15% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 15 | 1.15% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 16 | 1.00% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 17 | 0.95% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 18 | 0.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16569` |
| 19 | 0.76% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 20 | 0.72% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 21 | 0.67% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 22 | 0.67% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 23 | 0.67% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16310` |
| 24 | 0.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10726` |
| 25 | 0.57% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 26 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 27 | 0.53% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.53% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 29 | 0.53% | `java.util.HashMap.getNode(Object) line: 586` |
| 30 | 0.48% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 31 | 0.48% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 32 | 0.48% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 33 | 0.43% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 34 | 0.43% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 35 | 0.43% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2701` |
| 36 | 0.38% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 37 | 0.38% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13832` |
| 38 | 0.33% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 39 | 0.33% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 40 | 0.33% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2834` |
| 41 | 0.29% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 42 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 934` |
| 43 | 0.29% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2655` |
| 44 | 0.24% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 45 | 0.24% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 46 | 0.24% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 47 | 0.24% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5188` |
| 48 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 49 | 0.24% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 50 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 51 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 954` |
| 52 | 0.19% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 53 | 0.19% | `jdk.internal.classfile.impl.EntryMap.firstToken(int) line: 92` |
| 54 | 0.19% | `java.util.Comparator.lambda$comparingLong$6043328a$1(ToLongFunction, Object, Object) line: 515` |
| 55 | 0.19% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 56 | 0.19% | `java.lang.String.equals(Object) line: 1926` |
| 57 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 930` |
| 58 | 0.19% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 59 | 0.19% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5200` |
| 60 | 0.19% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 10184` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.11% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16288` |
| 2 | 4.87% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13849` |
| 3 | 4.82% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.20% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13782` |
| 5 | 2.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 6 | 2.10% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 7 | 1.67% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.58% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13802` |
| 9 | 1.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 10 | 1.38% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16295` |
| 11 | 1.34% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15218` |
| 13 | 1.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 1.24% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 15 | 1.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8478` |
| 16 | 1.24% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 17 | 1.15% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2207` |
| 18 | 1.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10821` |
| 19 | 0.95% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 20 | 0.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16569` |
| 21 | 0.86% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 22 | 0.76% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.67% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |
| 25 | 0.57% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 26 | 0.53% | `net.minecraft.world.entity.Entity.isPassenger() line: 3733` |
| 27 | 0.53% | `java.util.HashMap.containsKey(Object) line: 603` |
| 28 | 0.53% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 29 | 0.48% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 30 | 0.43% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

