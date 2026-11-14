# JVM hotspots (JFR ExecutionSample)

Samples: 2836

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.89% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 5.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12445` |
| 3 | 3.07% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 4 | 2.72% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 5 | 2.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7942` |
| 6 | 1.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.83% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 8 | 1.69% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.66% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.59% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.20% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 13 | 1.06% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12464` |
| 14 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 0.88% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 16 | 0.74% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 17 | 0.74% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 18 | 0.60% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 19 | 0.56% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12482` |
| 20 | 0.56% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1910` |
| 21 | 0.53% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 22 | 0.49% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 23 | 0.46% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 24 | 0.46% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 25 | 0.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 26 | 0.46% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 27 | 0.42% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 28 | 0.42% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 29 | 0.42% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 30 | 0.42% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 31 | 0.42% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 32 | 0.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12669` |
| 33 | 0.39% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 34 | 0.39% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 35 | 0.39% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1864` |
| 36 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 37 | 0.39% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 38 | 0.35% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 39 | 0.35% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 40 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 41 | 0.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12574` |
| 42 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 43 | 0.32% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10854` |
| 44 | 0.32% | `java.util.ArrayList$Itr.next() line: 1058` |
| 45 | 0.32% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10834` |
| 46 | 0.28% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 47 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 48 | 0.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 49 | 0.28% | `net.minecraft.world.entity.Entity.makeBoundingBox(Vec3) line: 841` |
| 50 | 0.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 51 | 0.28% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5200` |
| 52 | 0.28% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 53 | 0.28% | `jdk.internal.foreign.HeapMemorySegmentImpl$OfByte.unsafeGetBase() line: 114` |
| 54 | 0.25% | `net.minecraft.world.level.biome.Climate$RTree.lambda$comparator$0(int, boolean, Climate$RTree$Node) line: 405` |
| 55 | 0.25% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 56 | 0.25% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 57 | 0.25% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1890` |
| 58 | 0.21% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 59 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 740` |
| 60 | 0.21% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12669` |
| 2 | 5.64% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10851` |
| 3 | 3.46% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 4 | 3.17% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10784` |
| 5 | 2.68% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 7 | 2.40% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.83% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 10 | 1.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 11 | 1.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7968` |
| 12 | 1.27% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5533` |
| 13 | 1.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6606` |
| 14 | 1.13% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 15 | 1.06% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 16 | 0.88% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 17 | 0.85% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 18 | 0.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 19 | 0.74% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.71% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.60% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 22 | 0.56% | `java.math.BigInteger.multiplyToLen(int[], int, int[], int, int[]) line: 1844` |
| 23 | 0.56% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.53% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 25 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 26 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12470` |
| 27 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.46% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.46% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10812` |
| 30 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms

