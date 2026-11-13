# JVM hotspots (JFR ExecutionSample)

Samples: 3449

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.65% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11849` |
| 2 | 4.41% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.33% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 1.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 7 | 1.57% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7347` |
| 9 | 1.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.28% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 11 | 1.28% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 12 | 0.81% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 0.75% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 14 | 0.70% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1377` |
| 15 | 0.67% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 16 | 0.61% | `java.util.HashMap.getNode(Object) line: 586` |
| 17 | 0.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12073` |
| 18 | 0.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 0.46% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 20 | 0.43% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 21 | 0.43% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 22 | 0.43% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 23 | 0.41% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 24 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 25 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 26 | 0.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 27 | 0.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11978` |
| 28 | 0.41% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 29 | 0.38% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.38% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1892` |
| 31 | 0.35% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 32 | 0.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 33 | 0.35% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.32% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 35 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11868` |
| 36 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 37 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 38 | 0.29% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 39 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 41 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 42 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 43 | 0.26% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 44 | 0.26% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 45 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 46 | 0.23% | `jdk.internal.misc.Unsafe.allocateInstance(Class)` |
| 47 | 0.23% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 48 | 0.23% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 49 | 0.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2881` |
| 50 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 762` |
| 51 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 52 | 0.20% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 53 | 0.20% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 54 | 0.20% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 55 | 0.20% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 8943` |
| 56 | 0.20% | `java.util.ArrayList$Itr.next() line: 1058` |
| 57 | 0.20% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5200` |
| 58 | 0.20% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 59 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 60 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 954` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12073` |
| 2 | 4.29% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10256` |
| 3 | 3.71% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10189` |
| 4 | 3.16% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 5 | 2.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 6 | 1.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 1.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.45% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5123` |
| 9 | 1.33% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 11 | 1.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7373` |
| 12 | 1.28% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 13 | 1.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6194` |
| 14 | 1.10% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 15 | 0.78% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 16 | 0.75% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 17 | 0.72% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 18 | 0.72% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 19 | 0.70% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.70% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2645` |
| 21 | 0.67% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 22 | 0.55% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.52% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.52% | `java.util.HashMap.containsKey(Object) line: 603` |
| 25 | 0.46% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 26 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.moveEntity(Entity) line: 546` |
| 27 | 0.41% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 29 | 0.38% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 30 | 0.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms

