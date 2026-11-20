# JVM hotspots (JFR ExecutionSample)

Samples: 2531

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.81% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 3.16% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 3 | 3.16% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15343` |
| 4 | 2.61% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14477` |
| 5 | 2.21% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.09% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 7 | 2.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10127` |
| 8 | 1.58% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.50% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 11 | 1.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.22% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.19% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 995` |
| 14 | 0.99% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 15 | 0.75% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 16 | 0.71% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 17 | 0.67% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 18 | 0.63% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 19 | 0.55% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 20 | 0.55% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 21 | 0.55% | `java.util.HashMap.getNode(Object) line: 586` |
| 22 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 23 | 0.51% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2618` |
| 24 | 0.51% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 25 | 0.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 26 | 0.47% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 27 | 0.43% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 28 | 0.43% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 30 | 0.40% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 31 | 0.40% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 32 | 0.40% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 33 | 0.40% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 34 | 0.36% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 35 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 36 | 0.36% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 37 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.32% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 39 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 40 | 0.32% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 41 | 0.32% | `java.lang.Class.cast(Object) line: 3491` |
| 42 | 0.32% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 43 | 0.32% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5173` |
| 44 | 0.28% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 45 | 0.28% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 46 | 0.28% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 47 | 0.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 48 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 49 | 0.28% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 50 | 0.28% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 51 | 0.28% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 11806` |
| 52 | 0.28% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2587` |
| 53 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 54 | 0.24% | `net.minecraft.world.level.levelgen.Heightmap.lambda$static$0(BlockState) line: 28` |
| 55 | 0.24% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 56 | 0.24% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 57 | 0.24% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13102` |
| 58 | 0.24% | `java.lang.String.equals(Object) line: 1926` |
| 59 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 60 | 0.20% | `java.lang.StringLatin1.replace(byte[], char, char) line: 311` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.69% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13119` |
| 2 | 3.87% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13052` |
| 3 | 3.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15623` |
| 4 | 2.81% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 5 | 2.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 6 | 2.21% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.21% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.13% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 2.09% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 10 | 1.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10142` |
| 11 | 1.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 12 | 1.34% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 13 | 1.22% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 14 | 1.22% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14485` |
| 15 | 0.91% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 0.87% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 17 | 0.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8080` |
| 18 | 0.75% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.75% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 20 | 0.75% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 21 | 0.67% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 22 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 23 | 0.59% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.59% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15354` |
| 25 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 26 | 0.51% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 27 | 0.51% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 28 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15349` |
| 29 | 0.47% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 30 | 0.47% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |

GC pauses: 0, total 0 ms

